use std::sync::PoisonError;

use lancedb::arrow::arrow_schema::ArrowError;
use tauri::{ipc::InvokeError, App};
use tauri_plugin_http::reqwest;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
    #[error("Network error: connect to host:{0} failed")]
    NetworkError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("rights limit error: {0}")]
    RightsLimitError(String),
    #[error("image format error: {0}")]
    ImgFormatError(String),
}

impl From<tauri_plugin_store::Error> for AppError {
    fn from(e: tauri_plugin_store::Error) -> Self {
        AppError::InternalError(format!("tauri_plugin_store::Error: {:?}", e))
    }
}
impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        InvokeError::from_error(Box::new(self))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if let Some(url) = err.url() {
            AppError::NetworkError(url.to_string())
        } else {
            AppError::InternalError(format!("{:?}", err))
        }
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}

impl From<fast_image_resize::ResizeError> for AppError {
    fn from(err: fast_image_resize::ResizeError) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}

impl From<lancedb::Error> for AppError {
    fn from(err: lancedb::Error) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}

impl From<ArrowError> for AppError {
    fn from(err: ArrowError) -> Self {
        AppError::InternalError(format!("{:?}", err))
    }
}