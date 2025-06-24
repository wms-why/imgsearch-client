use std::sync::PoisonError;

use tauri::ipc::InvokeError;
use tauri_plugin_http::reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
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
        }else  {
            AppError::InternalError(format!("{:?}", err))
        }
    }
}

impl <T> From<PoisonError<T>> for AppError { 
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