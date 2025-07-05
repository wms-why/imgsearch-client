pub mod imgsearch_server;

use std::path::PathBuf;

use dotenvy::var;
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
    pub vec: Vec<f64>,
    pub desc: String,
    pub name: Option<String>,
}

pub trait ImageIndexer {
    async fn index(&self, path: PathBuf, rename: bool) -> Result<ImageIndexResp, AppError>;
    async fn indexes(
        &self,
        params: &[PathBuf],
        rename: bool,
    ) -> Result<Vec<ImageIndexResp>, AppError>;
}

pub fn init_server(app: &App) -> Result<Option<ImgseachServer>, AppError> {
    let auth = app.store("auth.json")?;
    let binding = auth.get("apikey");

    if let Some(binding) = binding {
        let apikey = binding.as_str();

        if let Some(apikey) = apikey {

            let host = env!("NEXT_PUBLIC_IMGSEARCH_HOST");
            Ok(Some(ImgseachServer::new(apikey.to_string(), host.into())))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
