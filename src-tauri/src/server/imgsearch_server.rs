use std::{path::PathBuf};

use tauri::http::StatusCode;
use tauri_plugin_http::reqwest;

use crate::{
    error::AppError,
    server::{ImageIndexResp, ImageIndexer},
};

pub struct ImgseachServer {
    apikey: String,
    host: String,
}
impl ImgseachServer {
    pub fn new(apikey: String) -> Self {
        Self {
            apikey,
            host: std::env::var("IMGSEARCH_HOST").unwrap(),
        }
    }
}

impl ImageIndexer for ImgseachServer {
    async fn index(
        &self,
        thumbnail_path: PathBuf,
        rename: bool,
    ) -> Result<ImageIndexResp, AppError> {
        let form = reqwest::multipart::Form::new()
            .file("thumbnail", thumbnail_path)
            .await?
            .text("rename", rename.to_string());

        let r = reqwest::Client::new()
            .post(format!("{}/api/image_index/v1", &self.host))
            .multipart(form)
            .send()
            .await?;

        if r.status().is_success() {
            Ok(r.json::<ImageIndexResp>().await?)
        } else {
            Err(judge_imgsearch_error(r.status()))
        }
    }
    
    async fn indexes(&self, params: &[PathBuf], rename: bool) -> Result<ImageIndexResp, AppError> {
        let mut form = reqwest::multipart::Form::new();

        let mut i = 0;
        for p in params.iter() {
            form = form.file(format!("thumbnail_{}", i), p.as_path().to_str().unwrap())
            .await?;
            i += 1;
        }

        form = form.text("rename", rename.to_string());

        let r = reqwest::Client::new()
            .post(format!("{}/api/image_indexes/v1", &self.host))
            .multipart(form)
            .send()
            .await?;

        if r.status().is_success() {
            Ok(r.json::<ImageIndexResp>().await?)
        } else {
            Err(judge_imgsearch_error(r.status()))
        }
    }
}

fn judge_imgsearch_error(status: StatusCode) -> AppError {
    match status {
        StatusCode::PRECONDITION_FAILED => {
            AppError::RightsLimitError("image_index count not enough".to_string())
        }
        StatusCode::UNAUTHORIZED => AppError::AuthError("apikey has been invalid".to_string()),
        _ => AppError::InternalError("unknown error".to_string()),
    }
}
