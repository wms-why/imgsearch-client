use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: u64,
}

#[tauri::command]
pub fn search_images(query: String, state: State<AppState>) -> Vec<ImageInfo> {
    let images = state.images.lock().unwrap();
    images
        .values()
        .filter(|img| img.name.to_lowercase().contains(&query.to_lowercase()))
        .cloned()
        .collect()
}

#[tauri::command]
pub async fn index_image(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<ImageInfo, String> {
    let source_path = Path::new(&file_path);

    // 验证文件是图片
    if let Some(extension) = source_path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if !["jpg", "jpeg", "png", "gif", "webp", "bmp"].contains(&ext.as_str()) {
            return Err("不支持的文件类型，仅支持图片格式".to_string());
        }
    } else {
        return Err("无法识别文件类型".to_string());
    }

    // 生成唯一ID和目标路径
    let id = Uuid::new_v4().to_string();
    let file_name = source_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let storage_path = state.storage_path.lock().unwrap().clone();
    let target_path = storage_path.join(&file_name);

    // 复制文件到应用存储目录
    fs::copy(source_path, &target_path).map_err(|e| format!("复制文件失败: {}", e))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let image_info = ImageInfo {
        id: id.clone(),
        name: file_name,
        path: target_path.to_string_lossy().to_string(),
        created_at: now,
    };

    // 更新状态
    let mut images = state.images.lock().unwrap();
    images.insert(id, image_info.clone());

    Ok(image_info)
}

#[tauri::command]
pub fn delete_image(id: String, state: State<AppState>) -> Result<(), String> {
    let mut images = state.images.lock().unwrap();

    if let Some(image) = images.remove(&id) {
        // 删除文件
        fs::remove_file(&image.path).map_err(|e| format!("删除文件失败: {}", e))?;
        Ok(())
    } else {
        Err("找不到指定图片".to_string())
    }
}
