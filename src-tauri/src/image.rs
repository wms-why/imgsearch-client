use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    error::AppError,
    image_idx, image_utils, path_utils,
    server::{ImageIndexResp, ImageIndexer},
    AppState,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: u64,
}

#[tauri::command]
pub fn search_images(query: String) -> Vec<ImageInfo> {
    // let images = state.images.lock().unwrap();
    // images
    //     .values()
    //     .filter(|img| img.name.to_lowercase().contains(&query.to_lowercase()))
    //     .cloned()
    //     .collect()

    Vec::new()
}

#[derive(Deserialize, Clone, Debug)]
pub struct ImageIdxModel {
    #[serde(rename = "rootDir")]
    pub root_dir: String,
    pub path: String,
    pub name: String,
    pub rename: bool,
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
    let mut t = Vec::with_capacity(model.paths.len());
    for path in model.paths.iter() {
        let source_bs = std::fs::read(Path::new(path))?;

        let format = image_utils::guess_format(source_bs.as_slice())?;
        let bs = image_utils::downscale(&source_bs, format)?;

        let thumbnail_path = match bs {
            Some(bs) => image_utils::save_local(bs.as_ref(), format)?,
            None => image_utils::save_local(&source_bs, format)?,
        };

        t.push(thumbnail_path);
    }

    let r = state.server.indexes(&t, model.rename).await?;

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
                        p.clone()
                    }
                } else {
                    p.clone()
                }
            })
            .collect::<Vec<_>>();

        model.paths = new_paths;
    }

    let idxes = model
        .paths
        .iter()
        .zip(r.into_iter())
        .map(|(p, ImageIndexResp { vec, desc, .. })| {
            let current_path = std::path::Path::new(p);
            image_idx::ImgIdx {
                name: current_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                path: p.to_string(),
                root: model.root_dir.clone(),
                desc,
                vec,
            }
        })
        .collect::<Vec<_>>();

    image_idx::save_batch(state.img_idx_tbl.clone(), idxes).await?;

    Ok(())
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
