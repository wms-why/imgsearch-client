use std::sync::PoisonError;

use tauri::ipc::InvokeError;
use tauri_plugin_http::reqwest;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network error: connect to host:{0} failed")]
    Network(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("image format error: {0}")]
    ImgFormat(String),
}

impl From<tauri_plugin_store::Error> for AppError {
    fn from(e: tauri_plugin_store::Error) -> Self {
        AppError::Internal(format!("tauri_plugin_store::Error: {:?}", e))
    }
}
impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        InvokeError::from_error(Box::new(self))
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        AppError::Internal(format!("{:?}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("{:?}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal(format!("{:?}", err))
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::Internal(format!("{:?}", err))
    }
}

impl From<fast_image_resize::ResizeError> for AppError {
    fn from(err: fast_image_resize::ResizeError) -> Self {
        AppError::Internal(format!("{:?}", err))
    }
}