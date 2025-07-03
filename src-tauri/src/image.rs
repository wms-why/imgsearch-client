use serde::{Deserialize, Serialize};

use crate::{error::AppError, image_utils};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThumbnailModel {
    pub source_path: String,
    pub target_path: String,
}

#[tauri::command]
pub async fn generate_thumbnail(model: ThumbnailModel) -> Result<(), AppError> {
    let s = std::fs::read(&model.source_path)?;
    let format = image_utils::guess_format(&s)?;
    let t = image_utils::downscale(&s, format)?;

    let buf = if let Some(t) = t {
        t.to_vec()
    } else {
        s
    };
    std::fs::write(&model.target_path, &buf)?;

    Ok(())
}
