use crate::state::AppState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::SqlitePool;
use tauri::State;

async fn get_password_hash(db: &SqlitePool) -> Result<Option<String>, String> {
    let row: Option<(String,)> = sqlx::query_as("SELECT password_hash FROM app_auth WHERE id = 1")
        .fetch_optional(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(row.map(|r| r.0))
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| e.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[tauri::command]
pub async fn has_password(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(get_password_hash(&state.db).await?.is_some())
}

#[tauri::command]
pub async fn set_initial_password(
    state: State<'_, AppState>,
    password: String,
) -> Result<(), String> {
    if password.trim().len() < 4 {
        return Err("La contraseña debe tener al menos 4 caracteres".into());
    }
    if get_password_hash(&state.db).await?.is_some() {
        return Err("Ya existe una contraseña configurada".into());
    }

    let hash = hash_password(&password)?;
    sqlx::query("INSERT INTO app_auth (id, password_hash) VALUES (1, ?)")
        .bind(hash)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    *state.authenticated.lock().unwrap() = true;
    Ok(())
}

#[tauri::command]
pub async fn login(state: State<'_, AppState>, password: String) -> Result<bool, String> {
    let Some(hash) = get_password_hash(&state.db).await? else {
        return Err("No hay contraseña configurada".into());
    };

    let valid = verify_password(&password, &hash)?;
    if valid {
        *state.authenticated.lock().unwrap() = true;
    }
    Ok(valid)
}

#[tauri::command]
pub fn logout(state: State<'_, AppState>) -> Result<(), String> {
    *state.authenticated.lock().unwrap() = false;
    Ok(())
}

#[tauri::command]
pub fn is_authenticated(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(*state.authenticated.lock().unwrap())
}

#[tauri::command]
pub async fn change_password(
    state: State<'_, AppState>,
    current_password: String,
    new_password: String,
) -> Result<(), String> {
    if new_password.trim().len() < 4 {
        return Err("La nueva contraseña debe tener al menos 4 caracteres".into());
    }

    let Some(hash) = get_password_hash(&state.db).await? else {
        return Err("No hay contraseña configurada".into());
    };
    if !verify_password(&current_password, &hash)? {
        return Err("Contraseña actual incorrecta".into());
    }

    let new_hash = hash_password(&new_password)?;
    let now = chrono::Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    sqlx::query("UPDATE app_auth SET password_hash = ?, updated_at = ? WHERE id = 1")
        .bind(&new_hash)
        .bind(now)
        .execute(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
