// Prevents additional console window on Windows in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{str::FromStr, sync::Arc};

use crate::server::init_server;
use futures::future::FutureExt;
use tauri::{async_runtime::RwLock, Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};
mod auth_api;
mod db;
mod error;
mod image_api;
mod path_utils;
mod server;
mod uuid_utils;

pub struct GlobalState {
    // pub cache:
    pub server: RwLock<Option<Arc<server::imgsearch_server::ImgseachServer>>>,
    pub auth_store: Arc<Store<Wry>>,
    pub imgdir_store: Arc<Store<Wry>>,
    pub img_idx_tbl: Arc<lancedb::Table>,
}

impl GlobalState {
    pub async fn set_server(&self, server: server::imgsearch_server::ImgseachServer) {
        let mut w = self.server.write().await;
        *w = Some(Arc::new(server));
    }
}

fn main() {
    if dotenvy::dotenv().is_err() {
        log::warn!("not .env fount");
    }

    let log_level = match std::env::var("RUST_LOG") {
        Ok(v) => v,
        Err(_) => "info".to_string(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        path: path_utils::logs_dir().expect("Failed to get logs dir"),
                        file_name: None,
                    },
                ))
                .level(log::LevelFilter::from_str(log_level.as_str()).unwrap())
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            image_api::search,
            image_api::rename,
            image_api::delete,
            image_api::modify_content,
            image_api::show_all,
            image_api::after_add_imgdir,
            image_api::after_remove_imgdir,
            auth_api::after_apikey_set
        ])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let img_idx_tbl = tauri::async_runtime::block_on(async { db::init_db().await })
                .expect("Failed to init db");

            let img_idx_tbl = Arc::new(img_idx_tbl);
            let auth_store = app.store("Auth.json")?;
            let imgdir_store = app.store("ImgDirStore.json")?;

            let server = RwLock::new(init_server(auth_store.clone())?.map(Arc::new));

            app.manage(GlobalState {
                server,
                img_idx_tbl,
                auth_store,
                imgdir_store,
            });

            let state = app.state::<GlobalState>();
            image_api::after_start_up(state).then(|r| {

            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
