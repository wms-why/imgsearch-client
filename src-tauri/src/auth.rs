use tauri::{App, State};

use crate::{error::AppError, server::init_server, AppState};

#[tauri::command]
pub async fn after_apikey_set(app: &App, state: State<'_, AppState>) -> Result<(), AppError> {
    let server = init_server(app)?;

    if let Some(server) = server {
        state.set_server(server);
    }

    Ok(())
}
