use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
    time::Duration,
};

use itertools::multizip;
use log::debug;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    error::AppError,
    image_idx::{self, update_path, update_path_prefix, ImgSearchResult},
    image_utils::{self, gen_thumbnail},
    path_utils,
    server::{ImageIndexResp, ImageIndexer},
    AppState,
};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: u64,
}

pub async fn index_images(
    root: &str,
    paths: Vec<PathBuf>,
    rename: bool,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    if state.server.read().await.is_none() {
        return Err(AppError::Auth("server not ready".to_string()));
    }

    let mut thumbnails = Vec::with_capacity(paths.len());
    let mut signs = Vec::with_capacity(paths.len());
    let cache = get_indexing_paths();

    for path in paths.iter() {
        let (sign, thumbnail_path) = gen_thumbnail(root, path)?;
        signs.push(sign);
        thumbnails.push(thumbnail_path);
    }

    let server = state.server.read().await;
    let server = server.as_ref().unwrap().clone();

    let paths = paths
        .iter()
        .filter(|p| cache.contains_key(p.as_path().to_str().unwrap()))
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return Ok(());
    }

    let r = server.indexes(&thumbnails, rename).await;

    let paths = paths
        .iter()
        .filter(|p| cache.contains_key(p.as_path().to_str().unwrap()))
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return Ok(());
    }

    let idxes = if let Ok(r) = r {
        let new_paths = if rename {
            let mut ps = Vec::with_capacity(paths.len());

            for (p, r) in paths.iter().zip(r.iter()) {
                ps.push(if let Some(newname) = &r.name {
                    let new_path = path_utils::rename(p.as_path(), newname);
                    if let Ok(new_path) = new_path {
                        new_path
                    } else {
                        log::error!("rename error: {}", new_path.err().unwrap());
                        p.to_path_buf()
                    }
                } else {
                    p.to_path_buf()
                });
            }

            ps
        } else {
            let mut ps = Vec::with_capacity(paths.len());
            for p in paths.iter() {
                if let Some(Some(p)) = cache.get(p.as_path().to_str().unwrap()).await {
                    ps.push(Path::new(p.as_str()).to_path_buf());
                } else {
                    ps.push(p.to_path_buf());
                }
            }
            ps
        };

        multizip((new_paths, signs, thumbnails, r.into_iter()))
            .map(|(p, sign, t, ImageIndexResp { vec, desc, .. })| {
                image_idx::ImgIdx::new(&p, root.to_string(), sign, t, desc, vec)
            })
            .collect::<Vec<_>>()
    } else {
        let err = r.unwrap_err();
        debug!("index_images error: {err:?}");

        multizip((&paths, signs, thumbnails))
            .map(|(p, sign, t)| {
                image_idx::ImgIdx::new_empty(p.as_path(), root.to_string(), sign, t.as_path())
            })
            .collect::<Vec<_>>()
    };

    for p in paths.iter() {
        cache.invalidate(p.as_path().to_str().unwrap()).await
    }

    image_idx::save_batch(state.img_idx_tbl.clone(), idxes).await?;

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
    state: State<'_, AppState>,
) -> Result<Vec<image_idx::ImgSearchResult>, AppError> {
    if state.server.read().await.is_none() {
        return Err(AppError::Auth("server not ready".to_string()));
    }
    let server = state.server.read().await;
    let server = server.as_ref().unwrap().clone();
    let r = server.text_vectorize(&model.keyword).await?;
    let r = image_idx::search(state.img_idx_tbl.clone(), &r, model.top).await?;
    Ok(r)
}

#[tauri::command]
pub async fn show_all(state: State<'_, AppState>) -> Result<Vec<ImgSearchResult>, AppError> {
    let r = image_idx::get_all(state.img_idx_tbl.clone()).await?;
    Ok(r)
}

#[tauri::command]
pub async fn after_add_imgdir(
    root: String,
    rename: bool,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let mut imgs = path_utils::find_all_images(Path::new(&root))?;

    let cache = get_indexing_paths();
    for p in imgs.iter() {
        cache.insert(p.display().to_string(), None).await;
    }

    loop {
        imgs = imgs
            .into_iter()
            .filter(|p| cache.contains_key(p.as_path().to_str().unwrap()))
            .collect::<Vec<_>>();

        if imgs.is_empty() {
            break;
        }

        let chunk = if imgs.len() >= 5 {
            imgs.drain(0..5)
        } else {
            imgs.drain(0..)
        };
        index_images(
            root.as_str(),
            chunk.collect::<Vec<_>>(),
            rename,
            state.clone(),
        )
        .await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn after_remove_imgdir(root: String, state: State<'_, AppState>) -> Result<(), AppError> {
    log::info!("remove img dir: {root}");

    image_idx::remove_by_root(state.img_idx_tbl.clone(), &root).await?;

    image_utils::remove_dir(&root)?;

    Ok(())
}

#[tauri::command]
pub async fn delete(path: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let path = Arc::new(path);
    let c = get_indexing_paths();
    let p = path.clone();
    let _ = c.invalidate_entries_if(move |key, _| key.starts_with(p.as_str()));

    let r = image_idx::remove_path_like(state.img_idx_tbl.clone(), path.as_str()).await?;

    if !r.is_empty() {
        for thumbnail in r {
            log::debug!("remove thumbnail: {}", thumbnail);
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
pub async fn rename(model: RenameModel, state: State<'_, AppState>) -> Result<(), AppError> {
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
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let mut paths = paths
        .into_iter()
        .filter(|p| path_utils::is_support_file(Path::new(p)))
        .collect::<Vec<_>>();
    let c = get_indexing_paths();
    for p in paths.iter() {
        c.insert(p.clone(), None).await;
    }

    loop {
        paths = paths
            .into_iter()
            .filter(|p| c.contains_key(p))
            .collect::<Vec<_>>();

        if paths.is_empty() {
            break;
        }
        let path = if paths.len() >= 5 {
            paths.drain(0..5)
        } else {
            paths.drain(0..)
        };

        index_images(
            &root,
            path.map(|p| Path::new(&p).to_path_buf())
                .collect::<Vec<_>>(),
            rename,
            state.clone(),
        )
        .await?;
    }

    Ok(())
}
