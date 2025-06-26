pub mod imgsearch_server;

use std::path::PathBuf;

/**
 * 定义大模型服务接口，目前只支持imgsearch
 */
use serde::Deserialize;
use tauri::App;
use tauri_plugin_store::StoreExt;

use crate::error::AppError;
use imgsearch_server::ImgseachServer;

#[derive(Deserialize)]
pub struct ImageIndexResp {
    vec: Vec<f32>,
    desc: String,
    name: Option<String>,
}

pub trait ImageIndexer {
    async fn index(&self, path: PathBuf, rename: bool) -> Result<ImageIndexResp, AppError>;
}

pub fn init_server(app: &App) -> Result<ImgseachServer, AppError> {
    let auth = app.store("auth.json")?;
    let binding = auth.get("apikey").unwrap();
    let apikey = binding.as_str().unwrap();
    Ok(ImgseachServer::new(apikey.to_string()))
}
