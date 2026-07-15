use crate::state::AppState;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};

fn app_database_path(app: &AppHandle) -> PathBuf {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir");
    app_dir.join("pvsr_accounts.db")
}

fn remove_sqlite_sidecars(path: &PathBuf) {
    for sidecar in [path.with_extension("db-wal"), path.with_extension("db-shm")] {
        if sidecar.exists() {
            let _ = std::fs::remove_file(&sidecar);
        }
    }
}

fn prepare_database_directory(path: &PathBuf) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("No se pudo preparar la carpeta de destino: {e}"))?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn export_database(app: AppHandle, destination: String) -> Result<String, String> {
    let source_path = app_database_path(&app);
    if !source_path.exists() {
        return Err("No existe una base de datos activa para exportar".into());
    }

    let destination_path = PathBuf::from(destination);
    prepare_database_directory(&destination_path)?;

    std::fs::copy(&source_path, &destination_path)
        .map_err(|e| format!("No se pudo exportar la base de datos: {e}"))?;

    Ok(format!(
        "Base de datos exportada correctamente en {}",
        destination_path.display()
    ))
}

#[tauri::command]
pub async fn import_database(
    state: State<'_, AppState>,
    app: AppHandle,
    source_path: String,
) -> Result<String, String> {
    let source = PathBuf::from(source_path);
    if !source.exists() {
        return Err("No se encontró el archivo seleccionado".into());
    }

    let destination_path = app_database_path(&app);
    prepare_database_directory(&destination_path)?;

    state.db.close().await;
    remove_sqlite_sidecars(&destination_path);
    if destination_path.exists() {
        let _ = std::fs::remove_file(&destination_path);
    }

    std::fs::copy(&source, &destination_path)
        .map_err(|e| format!("No se pudo importar la base de datos: {e}"))?;
    remove_sqlite_sidecars(&destination_path);
    remove_sqlite_sidecars(&source);

    Ok(format!(
        "Base de datos importada correctamente desde {}",
        source.display()
    ))
}
