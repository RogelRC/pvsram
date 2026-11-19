use crate::error::{AppError, AppResult};
use crate::state::AppState;
use chrono::Local;
use tauri::State;

fn local_timestamp() -> String {
    Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

fn validate_window_mode(mode: &str) -> AppResult<()> {
    match mode {
        "fullscreen" | "maximized" | "windowed" => Ok(()),
        _ => Err(AppError {
            message: format!(
                "Modo de ventana inválido: '{mode}'. Use fullscreen, maximized o windowed."
            ),
        }),
    }
}

#[tauri::command]
pub async fn get_window_mode(state: State<'_, AppState>) -> AppResult<String> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT window_mode FROM app_settings WHERE id = 1")
            .fetch_optional(&state.db)
            .await?;

    Ok(row.map(|r| r.0).unwrap_or_else(|| "fullscreen".to_string()))
}

#[tauri::command]
pub async fn set_window_mode(state: State<'_, AppState>, mode: String) -> AppResult<()> {
    validate_window_mode(&mode)?;
    let now = local_timestamp();
    sqlx::query(
        "INSERT INTO app_settings (id, window_mode, created_at, updated_at)
         VALUES (1, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
            window_mode = excluded.window_mode,
            updated_at = excluded.updated_at",
    )
    .bind(&mode)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(())
}
