use std::path::Path;

use image::ImageFormat;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{error::AppError, image_utils, server::ImageIndexer, AppState};

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

#[derive(Serialize, Clone, Debug)]
pub struct ImageIdxModel {
    #[serde(rename = "rootDir")]
    pub root_dir: String,
    pub path: String,
    pub name: String,
    pub rename: bool,
}

impl ImageIdxModel {
    fn get_subfix(&self) -> &str {
        &self.name[self.name.rfind(".").unwrap() + 1..]
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn index_image(model: ImageIdxModel, state: State<'_, AppState>) -> Result<(), AppError> {
    let source_bs = std::fs::read(Path::new(&model.path))?;

    let format = image_utils::guess_format(source_bs.as_slice())?;
    let bs = image_utils::downscale(&source_bs, format)?;

    let thumbnail_path = match bs {
        Some(bs) => image_utils::save_local(bs.as_ref(), format)?,
        None => image_utils::save_local(&source_bs, format)?,
    };

    let index_info = state.server.index(thumbnail_path, model.rename).await?;

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
