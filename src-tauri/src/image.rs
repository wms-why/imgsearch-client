use std::path::Path;

use log::debug;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    error::AppError,
    image_idx::{self, update_path, update_path_prefix},
    image_utils, path_utils,
    server::{ImageIndexResp, ImageIndexer},
    uuid_utils, AppState,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageIdxModel {
    #[serde(rename = "rootDir")]
    root_dir: String,
    path: String,
    name: String,
    rename: bool,
}

// #[tauri::command(rename_all = "snake_case")]
// pub async fn index_image(model: ImageIdxModel, state: State<'_, AppState>) -> Result<(), AppError> {
//     let source_bs = std::fs::read(Path::new(&model.path))?;

//     let format = image_utils::guess_format(source_bs.as_slice())?;
//     let bs = image_utils::downscale(&source_bs, format)?;

//     let thumbnail_path = match bs {
//         Some(bs) => image_utils::save_local(bs.as_ref(), format)?,
//         None => image_utils::save_local(&source_bs, format)?,
//     };

//     let index_info = state.server.index(thumbnail_path, model.rename).await?;

//     Ok(())
// }

#[derive(Deserialize, Clone, Debug)]
pub struct ImageIdxModels {
    #[serde(rename = "rootDir")]
    pub root_dir: String,
    pub paths: Vec<String>,
    pub rename: bool,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn index_images(
    mut model: ImageIdxModels,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    if state.server.read().await.is_none() {
        return Err(AppError::Auth("server not ready".to_string()));
    }

    // let sever = state.server.as_ref().unwrap();

    let mut thumbnails = Vec::with_capacity(model.paths.len());
    for path in model.paths.iter() {
        let source_bs = std::fs::read(Path::new(path))?;

        let format = image_utils::guess_format(source_bs.as_slice())?;
        let bs = image_utils::downscale(&source_bs, format)?;

        let thumbnail_path = match bs {
            Some(bs) => image_utils::save_local(bs.as_ref(), format)?,
            None => image_utils::save_local(&source_bs, format)?,
        };

        thumbnails.push(thumbnail_path);
    }

    let server = state.server.read().await;
    let server = server.as_ref().unwrap().clone();
    let r = server.indexes(&thumbnails, model.rename).await;

    let idxes = if let Ok(r) = r {
        if model.rename {
            let new_paths = model
                .paths
                .iter()
                .zip(r.iter())
                .map(|(p, r)| {
                    if let Some(newname) = &r.name {
                        if let Ok(new_path) = path_utils::rename(p, newname) {
                            new_path.to_str().unwrap().to_string()
                        } else {
                            p.to_string()
                        }
                    } else {
                        p.to_string()
                    }
                })
                .collect::<Vec<_>>();

            model.paths = new_paths;
        }

        model
            .paths
            .iter()
            .zip(thumbnails.iter())
            .zip(r.into_iter())
            .map(|((p, t), ImageIndexResp { vec, desc, .. })| {
                let current_path = std::path::Path::new(p.as_str());
                image_idx::ImgIdx::new(
                    current_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    p.to_string(),
                    model.root_dir.clone(),
                    t.as_path().to_str().unwrap().to_string(),
                    desc,
                    vec,
                )
            })
            .collect::<Vec<_>>()
    } else {
        let err = r.unwrap_err();
        debug!("index_images error: {err:?}");

        model
            .paths
            .iter()
            .zip(thumbnails.iter())
            .map(|(p, t)| {
                let current_path = std::path::Path::new(p.as_str());
                image_idx::ImgIdx::new_empty(
                    current_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    p.to_string(),
                    model.root_dir.clone(),
                    t.as_path().to_str().unwrap().to_string(),
                )
            })
            .collect::<Vec<_>>()
    };

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
pub fn delete_image(id: String) -> Result<(), String> {
    // if let Some(image) = images.remove(&id) {
    //     // 删除文件
    //     fs::remove_file(&image.path).map_err(|e| format!("删除文件失败: {}", e))?;
    //     Ok(())
    // } else {
    //     Err("找不到指定图片".to_string())
    // }
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
        update_path(state.img_idx_tbl.clone(), &model.old, &model.new).await?;
    } else if new.is_dir() {
        update_path_prefix(state.img_idx_tbl.clone(), &model.old, &model.new).await?;
    }
    Ok(())
}

#[tauri::command]
pub fn create_files(model: Vec<String>, state: State<'_, AppState>) -> Result<(), String> {
    // if let Some(image) = images.remove(&id) {
    //     // 删除文件
    //     fs::remove_file(&image.path).map_err(|e| format!("删除文件失败: {}", e))?;
    //     Ok(())
    // } else {
    //     Err("找不到指定图片".to_string())
    // }
    Ok(())
}
