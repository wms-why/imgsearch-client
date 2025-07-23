mod idx;
mod utils;

use std::{
    path::Path,
    sync::{Arc, OnceLock},
    time::Duration,
};

use itertools::{multizip, Itertools};
use moka::future::Cache;
use serde::Deserialize;
use tauri::State;

use crate::{
    error::AppError,
    path_utils,
    server::{ImageIndexResp, ImageIndexer},
    GlobalState,
};

pub use idx::get_table;
use idx::{update_path, update_path_prefix, ImgSearchResult, IndexModel};
use utils::gen_thumbnail;

#[warn(dead_code)]
#[derive(Deserialize)]
struct ImgDir {
    name: String,
    root: String,
    #[serde(alias = "enableRename")]
    rename: bool,
}

pub async fn after_start_up(state: State<'_, GlobalState>) -> Result<(), AppError> {
    let table = state.img_idx_tbl.clone();
    let imgdir_store = state.imgdir_store.clone();

    let cache = get_indexing_paths();

    let all: Vec<ImgSearchResult> = idx::get_all(table.clone(), Some(false)).await?;
    for p in all.iter() {
        cache.insert(p.path.clone(), None).await;
    }
    let r = all.into_iter().into_group_map_by(|r| r.root.clone());

    for (root, paths) in r.into_iter() {
        let imgdir_store = imgdir_store.clone();
        let state = state.clone();
        let r = imgdir_store.get(&root);

        if r.is_none() {
            // 说明有残余的图片索引
            log::warn!("root={} not found in imgdir_store", &root);
            idx::remove_path_like(table.clone(), &root).await?;
            continue;
        }

        let imgdir = serde_json::from_value::<ImgDir>(r.unwrap().take())?;

        let mut r = Vec::new(); // (id, path, thumbnail, root)
        for chunks in paths.chunks(5) {
            let ipt = &mut chunks
                .iter()
                .map(|c| {
                    (
                        c.id.clone(),
                        c.path.clone(),
                        c.thumbnail.clone(),
                        c.root.clone(),
                    )
                })
                .collect::<Vec<_>>();

            r.append(ipt);
        }

        loop {
            r = r
                .into_iter()
                .filter(|p| cache.contains_key(p.1.as_str()))
                .collect::<Vec<_>>();

            if r.is_empty() {
                break;
            }

            let chunk = if r.len() >= 5 {
                r.drain(0..5)
            } else {
                r.drain(0..)
            };

            let ipt = chunk.map(|c| (c.0, c.1, c.2)).collect::<Vec<_>>();

            index_images(&ipt, imgdir.rename, state.clone()).await?;

            for p in ipt.iter() {
                cache.invalidate(p.1.as_str()).await;
            }
        }

        if !r.is_empty() {
            for p in r.iter() {
                cache.invalidate(p.1.as_str()).await;
            }
        }
    }

    Ok(())
}
/**
 * 当前 正在索引 的路径缓存，key = old_path, value =  manual_new_path
 */
static INDEXING_PATHS: OnceLock<Cache<String, Option<String>>> = OnceLock::new();

fn get_indexing_paths() -> &'static Cache<String, Option<String>> {
    INDEXING_PATHS.get_or_init(|| {
        Cache::builder()
            .support_invalidation_closures()
            .time_to_live(Duration::from_secs(60))
            .build()
    })
}

async fn save_empty_image(
    root: &str,
    paths: Vec<&Path>,
    state: State<'_, GlobalState>,
) -> Result<Vec<(String, String, String)>, AppError> {
    let mut thumbnails = Vec::with_capacity(paths.len());
    let mut signs = Vec::with_capacity(paths.len());

    for path in paths.iter() {
        let (sign, thumbnail_path) = gen_thumbnail(root, path)?;
        signs.push(sign);
        thumbnails.push(thumbnail_path);
    }

    let idxes = multizip((&paths, signs, &thumbnails))
        .map(|(p, sign, t)| idx::ImgIdx::new_empty(p, root.to_string(), sign, t.as_path()))
        .collect::<Vec<_>>();

    let ids = idxes.iter().map(|i| i.id.clone()).collect::<Vec<_>>();

    idx::save_batch(state.img_idx_tbl.clone(), idxes).await?;

    Ok(multizip((ids, &paths, &thumbnails))
        .map(|(id, p, t)| (id, p.display().to_string(), t.display().to_string()))
        .collect::<Vec<_>>())
}
async fn index_images(
    ipt: &Vec<(String, String, String)>, // id, path, thumbnail
    rename: bool,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    if state.server.read().await.is_none() {
        return Err(AppError::Auth("server not ready".to_string()));
    }

    let server = state.server.read().await;
    let server = server.as_ref().unwrap().clone();

    let cache = get_indexing_paths();

    let ipt = ipt
        .iter()
        .filter(|p| cache.contains_key(&p.1))
        .collect::<Vec<_>>();

    if ipt.is_empty() {
        return Ok(());
    }

    let thumbnails = ipt.iter().map(|(_, _, t)| Path::new(t)).collect::<Vec<_>>();

    let r = server.indexes(thumbnails, rename).await?;

    let ipt = ipt
        .iter()
        .filter(|p| cache.contains_key(&p.1))
        .collect::<Vec<_>>();

    if ipt.is_empty() {
        return Ok(());
    }

    let id_path = if rename {
        let mut ps = Vec::with_capacity(ipt.len());

        for (p, r) in ipt.iter().zip(r.iter()) {
            let cc = cache.get(&p.0).await;
            let old_path = if let Some(Some(ref pp)) = cc {
                pp
            } else {
                &p.0
            };

            let new_path = if let Some(newname) = &r.name {
                let new_path = path_utils::rename(Path::new(old_path), newname);
                if let Ok(new_path) = new_path {
                    new_path.display().to_string()
                } else {
                    log::error!("rename error: {}", new_path.err().unwrap());
                    old_path.to_string()
                }
            } else {
                p.1.clone()
            };

            ps.push((&p.0, new_path));
        }

        ps
    } else {
        let mut ps = Vec::with_capacity(ipt.len());
        for p in ipt.iter() {
            let cc = cache.get(&p.0).await;
            if let Some(Some(ref new_path)) = cc {
                ps.push((&p.0, new_path.clone()));
            } else {
                ps.push((&p.0, p.1.to_string()));
            }
        }
        ps
    };

    let r = multizip((id_path, r.into_iter()))
        .map(|(p, ImageIndexResp { vec, desc, .. })| {
            let path = Path::new(&p.1);
            let filename = path.file_name().unwrap().display().to_string();

            IndexModel {
                id: p.0.to_string(),
                name: filename,
                path: p.1.to_string(),
                desc,
                vec,
            }
        })
        .collect::<Vec<_>>();

    idx::save_indexes(state.img_idx_tbl.clone(), r).await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct SearchModel {
    keyword: String,
    top: usize,
}
#[tauri::command]
pub async fn search(
    model: SearchModel,
    state: State<'_, GlobalState>,
) -> Result<Vec<idx::ImgSearchResult>, AppError> {
    if state.server.read().await.is_none() {
        return Err(AppError::Auth("server not ready".to_string()));
    }
    let server = state.server.read().await;
    let server = server.as_ref().unwrap().clone();
    let r = server.text_vectorize(&model.keyword).await?;
    let r = idx::search(state.img_idx_tbl.clone(), &r, model.top).await?;
    Ok(r)
}

#[tauri::command]
pub async fn show_all(state: State<'_, GlobalState>) -> Result<Vec<ImgSearchResult>, AppError> {
    let r = idx::get_all(state.img_idx_tbl.clone(), Some(true)).await?;
    Ok(r)
}

#[tauri::command]
pub async fn after_add_imgdir(
    root: String,
    rename: bool,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    let imgs = path_utils::find_all_images(Path::new(&root))?;

    let cache = get_indexing_paths();
    for p in imgs.iter() {
        cache.insert(p.display().to_string(), None).await;
    }

    let mut r: Vec<(String, String, String)> = Vec::new();
    for chunks in imgs.chunks(5) {
        let ipt = &mut (save_empty_image(
            &root,
            chunks.iter().map(|p| p.as_path()).collect(),
            state.clone(),
        )
        .await?);

        r.append(ipt);
    }

    loop {
        r = r
            .into_iter()
            .filter(|p| cache.contains_key(p.1.as_str()))
            .collect::<Vec<_>>();

        if r.is_empty() {
            break;
        }

        let chunk = if r.len() >= 5 {
            r.drain(0..5)
        } else {
            r.drain(0..)
        };

        let paths = chunk
            .as_ref()
            .iter()
            .map(|p| Path::new(&p.1))
            .collect::<Vec<_>>();
        let ipt = save_empty_image(&root, paths, state.clone()).await?;

        index_images(&ipt, rename, state.clone()).await?;

        for p in ipt.iter() {
            cache.invalidate(p.1.as_str()).await;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn after_remove_imgdir(
    root: String,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    log::info!("remove img dir: {root}");

    idx::remove_by_root(state.img_idx_tbl.clone(), &root).await?;

    utils::remove_dir(&root)?;

    Ok(())
}

#[tauri::command]
pub async fn delete(path: String, state: State<'_, GlobalState>) -> Result<(), AppError> {
    let path = Arc::new(path);
    let c = get_indexing_paths();
    let p = path.clone();
    let _ = c.invalidate_entries_if(move |key, _| key.starts_with(p.as_str()));

    let r = idx::remove_path_like(state.img_idx_tbl.clone(), path.as_str()).await?;

    if !r.is_empty() {
        for thumbnail in r {
            log::debug!("remove thumbnail: {thumbnail}");
            path_utils::remove_file(Path::new(&thumbnail))?;
        }
    }
    Ok(())
}

/**
 * 重命名文件或者文件夹
 */
#[derive(Deserialize)]
pub struct RenameModel {
    old: String,
    new: String,
}
#[tauri::command]
pub async fn rename(model: RenameModel, state: State<'_, GlobalState>) -> Result<(), AppError> {
    let new = Path::new(&model.new);
    if new.is_file() {
        // 过滤index的自动重命名所触发事件
        let c = get_indexing_paths();

        if (c.get(&model.old).await).is_some() {
            c.insert(model.old, Some(model.new)).await;
        } else {
            update_path(state.img_idx_tbl.clone(), &model.old, &model.new).await?;
        }
    } else if new.is_dir() {
        update_path_prefix(state.img_idx_tbl.clone(), &model.old, &model.new).await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn modify_content(
    root: String,
    paths: Vec<String>,
    rename: bool,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    let paths = paths
        .into_iter()
        .filter(|p| path_utils::is_support_file(Path::new(p)))
        .collect::<Vec<_>>();
    let cache = get_indexing_paths();
    for p in paths.iter() {
        cache.insert(p.clone(), None).await;
    }

    let mut r: Vec<(String, String, String)> = Vec::new();
    for chunks in paths.chunks(5) {
        let ipt =
            &mut (save_empty_image(&root, chunks.iter().map(Path::new).collect(), state.clone())
                .await?);

        r.append(ipt);
    }

    loop {
        r = r
            .into_iter()
            .filter(|p| cache.contains_key(p.1.as_str()))
            .collect::<Vec<_>>();

        if r.is_empty() {
            break;
        }

        let chunk = if r.len() >= 5 {
            r.drain(0..5)
        } else {
            r.drain(0..)
        };

        let paths = chunk
            .as_ref()
            .iter()
            .map(|p| Path::new(&p.1))
            .collect::<Vec<_>>();
        let ipt = save_empty_image(&root, paths, state.clone()).await?;

        index_images(&ipt, rename, state.clone()).await?;

        for p in chunk.as_ref() {
            cache.invalidate(p.1.as_str()).await;
        }
    }

    Ok(())
}
