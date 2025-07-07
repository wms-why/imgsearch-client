use tauri::State;

use crate::{error::AppError, server::init_server, AppState};

#[tauri::command]
pub async fn after_apikey_set(state: State<'_, AppState>) -> Result<(), AppError> {
    log::debug!("after_apikey_set");
    let auth_store = state.auth_store.clone();
    let server = init_server(auth_store.clone())?;

    if let Some(server) = server {
        state.set_server(server).await;
        Ok(())
    } else {
        let apikey = auth_store.get("apikey");

        if let Some(apikey) = apikey {
            Err(AppError::Auth(format!(
                "Failed to init server, apikey = {apikey} is invalid"
            )))
        } else {
            Err(AppError::Auth(
                "Failed to init server, apikey lack".to_string(),
            ))
        }
    }
}
