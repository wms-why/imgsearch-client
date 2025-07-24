// Prevents additional console window on Windows in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{str::FromStr, sync::Arc};

use crate::server::init_server;
use tauri::{async_runtime::RwLock, Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};
mod auth_command;
mod db;
mod error;
mod image_command;
mod path_utils;
mod server;
mod uuid_utils;
use server::imgsearch_server::ImgseachServer;
pub struct GlobalState {
    // pub cache:
    pub server: Arc<RwLock<Option<ImgseachServer>>>,
    pub auth_store: Arc<Store<Wry>>,
    pub imgdir_store: Arc<Store<Wry>>,
    pub img_idx_tbl: Arc<lancedb::Table>,
}

impl GlobalState {
    pub async fn set_server(&self, server: ImgseachServer) {
        let mut w = self.server.write().await;
        *w = Some(server);
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
            image_command::search,
            image_command::rename,
            image_command::delete,
            image_command::modify_content,
            image_command::show_all,
            image_command::after_add_imgdir,
            image_command::after_remove_imgdir,
            auth_command::after_apikey_set
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

            let server = Arc::new(RwLock::new(init_server(auth_store.clone())?));

            app.manage(GlobalState {
                server: server.clone(),
                img_idx_tbl: img_idx_tbl.clone(),
                auth_store: auth_store.clone(),
                imgdir_store: imgdir_store.clone(),
            });

            image_command::after_start_up(img_idx_tbl, imgdir_store, server);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
