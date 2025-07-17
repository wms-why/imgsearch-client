use std::path::PathBuf;

use tauri::http::StatusCode;
use tauri_plugin_http::reqwest::{self, Response};

use crate::{
    error::AppError,
    server::{ImageIndexResp, ImageIndexer},
};

pub struct ImgseachServer {
    apikey: String,
    host: String,
    client: reqwest::Client,
}
impl ImgseachServer {
    pub fn new(apikey: String, host: String) -> Self {
        Self {
            apikey,
            host,
            client: reqwest::Client::new(),
        }
    }
}

impl ImageIndexer for ImgseachServer {
    // async fn index(
    //     &self,
    //     thumbnail_path: PathBuf,
    //     rename: bool,
    // ) -> Result<ImageIndexResp, AppError> {
    //     let form = reqwest::multipart::Form::new()
    //         .file("thumbnail", thumbnail_path)
    //         .await?
    //         .text("rename", rename.to_string());

    //     let r = self
    //         .client
    //         .post(format!("{}/api/image_index/v1", &self.host))
    //         .bearer_auth(&self.apikey)
    //         .multipart(form)
    //         .send()
    //         .await?;

    //     if r.status().is_success() {
    //         Ok(r.json::<ImageIndexResp>().await?)
    //     } else {
    //         Err(judge_imgsearch_error(r).await)
    //     }
    // }

    async fn indexes(
        &self,
        params: &[PathBuf],
        rename: bool,
    ) -> Result<Vec<ImageIndexResp>, AppError> {
        let mut form = reqwest::multipart::Form::new();

        for (i, p) in params.iter().enumerate() {
            form = form
                .file(format!("thumbnail_{i}"), p.as_path().to_str().unwrap())
                .await?;
        }

        form = form.text("rename", rename.to_string());

        let r: Response = self
            .client
            .post(format!("{}/api/image_indexes/v1", &self.host))
            .bearer_auth(&self.apikey)
            .multipart(form)
            .send()
            .await?;

        if r.status().is_success() {
            Ok(r.json::<Vec<ImageIndexResp>>().await?)
        } else {
            Err(judge_imgsearch_error(r).await)
        }
    }

    async fn text_vectorize(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let r = self
            .client
            .get(format!("{}/api/text_vectorize/v1?text={}", &self.host, text))
            .bearer_auth(&self.apikey)
            .send()
            .await?;

        if r.status().is_success() {
            let json: Vec<f32> = r.json().await?;
            Ok(json)
        } else {
            Err(judge_imgsearch_error(r).await)
        }
    }
}

async fn judge_imgsearch_error(r: Response) -> AppError {
    let status = r.status();

    match status {
        StatusCode::PRECONDITION_FAILED => {
            AppError::RightsLimit("image_index count not enough".to_string())
        }
        StatusCode::UNAUTHORIZED => AppError::Auth("apikey has been invalid".to_string()),
        _ => {
            let msg = r.text().await;

            if let Err(e) = msg {
                AppError::from(e)
            } else {
                let msg = msg.unwrap();
                AppError::Internal(format!("unknown error, status: {status}, message: {msg}",))
            }
        }
    }
}
