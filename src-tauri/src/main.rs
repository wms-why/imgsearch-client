// Prevents additional console window on Windows in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use tauri::{async_runtime::{ RwLock}, Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};

use crate::server::init_server;

mod auth;
mod db;
mod error;
mod image;
mod image_idx;
mod image_utils;
mod path_utils;
mod server;
mod uuid_utils;

pub struct AppState {
    pub server: RwLock<Option<Arc<server::imgsearch_server::ImgseachServer>>>,
    pub auth_store: Arc<Store<Wry>>,
    pub img_idx_tbl: Arc<lancedb::Table>,
}

impl AppState {
    pub async fn set_server(&self, server: server::imgsearch_server::ImgseachServer) {
        let mut w = self.server.write().await;
        *w = Some(Arc::new(server));
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        path: path_utils::logs_dir().expect("Failed to get logs dir"),
                        file_name: None,
                    },
                ))
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            // guide::save_guide,
            // image::search_images
            image::index_images,
            auth::after_apikey_set
        ])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let img_idx_tbl = tauri::async_runtime::block_on(async { db::init_db().await })
                .expect("Failed to init db");

            let auth_store = app.store("Auth.json")?;

            let server = RwLock::new(init_server(auth_store.clone())?.map(Arc::new));

            app.manage(AppState {
                server,
                img_idx_tbl: Arc::new(img_idx_tbl),
                auth_store,
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
