use std::sync::PoisonError;

use lancedb::arrow::arrow_schema::ArrowError;
use tauri::ipc::InvokeError;
use tauri_plugin_http::reqwest;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network error: connect to host:{0} failed")]
    Network(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("rights limit error: {0}")]
    RightsLimit(String),
    #[error("image format error: {0}")]
    ImgFormat(String),
}

impl From<tauri_plugin_store::Error> for AppError {
    fn from(e: tauri_plugin_store::Error) -> Self {
        AppError::Internal(format!("tauri_plugin_store::Error: {e}"))
    }
}
impl From<AppError> for InvokeError {
    fn from(e: AppError) -> InvokeError {
        InvokeError::from_error(Box::new(e))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(url) = err.url() {
            AppError::Network(url.to_string())
        } else {
            AppError::Internal(format!("{err}"))
        }
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        AppError::Internal(format!("{err}"))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("{err}"))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal(format!("{err}"))
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::Internal(format!("{err}"))
    }
}

impl From<fast_image_resize::ResizeError> for AppError {
    fn from(err: fast_image_resize::ResizeError) -> Self {
        AppError::Internal(format!("{err}"))
    }
}

impl From<lancedb::Error> for AppError {
    fn from(err: lancedb::Error) -> Self {
        AppError::Internal(format!("{err}"))
    }
}

impl From<ArrowError> for AppError {
    fn from(err: ArrowError) -> Self {
        AppError::Internal(format!("{err}"))
    }
}
