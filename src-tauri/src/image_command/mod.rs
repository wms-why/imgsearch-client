mod api;
mod idx;
mod utils;

use std::sync::Arc;

use lancedb::Table;
use serde::Deserialize;
use tauri::{async_runtime::RwLock, State, Wry};
use tauri_plugin_store::Store;

use crate::{error::AppError, server::imgsearch_server::ImgseachServer, GlobalState};

pub use idx::get_table;
use idx::ImgSearchResult;

#[warn(dead_code)]
#[derive(Deserialize)]
struct ImgDir {
    name: String,
    root: String,
    #[serde(alias = "enableRename")]
    rename: bool,
}

pub fn after_start_up(
    table: Arc<Table>,
    imgdir_store: Arc<Store<Wry>>,
    server: Arc<RwLock<Option<ImgseachServer>>>,
) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = api::on_start_up(table, imgdir_store).await {
            log::error!("on_startup process error, {e:?}");
        }
    });
}

#[derive(Deserialize)]
pub struct SearchModel {
    keyword: String,
    top: usize,
}
#[tauri::command]
pub async fn search(
    model: SearchModel,
    state: State<'_, GlobalState>,
) -> Result<Vec<idx::ImgSearchResult>, AppError> {
    let server = state.server.read().await;
    if server.is_none() {
        return Err(AppError::Auth("server not ready".to_string()));
    }

    let server = server.as_ref().unwrap();
    Ok(api::search(&model, server, state.img_idx_tbl.clone()).await?)
}

#[tauri::command]
pub async fn show_all(state: State<'_, GlobalState>) -> Result<Vec<ImgSearchResult>, AppError> {
    let r = idx::get_all(state.img_idx_tbl.clone(), Some(true)).await?;
    Ok(r)
}

#[tauri::command]
pub async fn after_add_imgdir(
    root: String,
    rename: bool,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    let server = state.server.read().await;
    let server = if let Some(server) = server.as_ref() {
        Some(server)
    } else {
        None
    };

    api::index_imgdir(root, rename, server, state.img_idx_tbl.clone()).await?;

    Ok(())
}

#[tauri::command]
pub async fn after_remove_imgdir(
    root: String,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    log::info!("remove img dir: {root}");

    api::remove_root(&root, state.img_idx_tbl.clone()).await?;

    Ok(())
}

#[tauri::command]
pub async fn delete(path: String, state: State<'_, GlobalState>) -> Result<(), AppError> {
    api::delete_path(path, state.img_idx_tbl.clone()).await?;
    Ok(())
}

/**
 * 重命名文件或者文件夹
 */
#[derive(Deserialize)]
pub struct RenameModel {
    old: String,
    new: String,
}
#[tauri::command]
pub async fn rename(model: RenameModel, state: State<'_, GlobalState>) -> Result<(), AppError> {
    api::rename(model, state.img_idx_tbl.clone()).await?;
    Ok(())
}

#[tauri::command]
pub async fn modify_content(
    root: String,
    paths: Vec<String>,
    rename: bool,
    state: State<'_, GlobalState>,
) -> Result<(), AppError> {
    let server = state.server.read().await;
    let server = if let Some(server) = server.as_ref() {
        Some(server)
    } else {
        None
    };
    api::modify_content(root, paths, rename, server, state.img_idx_tbl.clone()).await?;

    Ok(())
}
