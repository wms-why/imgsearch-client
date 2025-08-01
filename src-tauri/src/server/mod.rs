pub mod imgsearch_server;

use std::{path::Path, sync::Arc};

/**
 * 定义大模型服务接口，目前只支持imgsearch
 */
use serde::Deserialize;
use tauri::Wry;
use tauri_plugin_store::Store;

use crate::error::AppError;
use imgsearch_server::ImgseachServer;

#[derive(Deserialize, Debug)]
pub struct ImageIndexResp {
    pub vec: Vec<f32>,
    pub desc: String,
    pub name: Option<String>,
}

pub trait ImageIndexer {
    async fn indexes(
        &self,
        params: Vec<&Path>,
        rename: bool,
    ) -> Result<Vec<ImageIndexResp>, AppError>;

    async fn text_vectorize(&self, text: &str) -> Result<Vec<f32>, AppError>;
}

pub fn init_server(auth_store: Arc<Store<Wry>>) -> Result<Option<ImgseachServer>, AppError> {
    let binding = auth_store.get("apikey");
    log::debug!("init_server, apikey: {binding:?}");

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
