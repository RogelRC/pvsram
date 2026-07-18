use crate::error::{AppError, AppResult};
use crate::models::concept::Concept;
use crate::state::AppState;
use chrono::Local;
use sqlx::SqlitePool;
use tauri::State;

const CONCEPT_COLUMNS: &str =
    "id, account_id, type, name, is_active, created_at, updated_at";

fn local_timestamp() -> String {
    Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

fn validate_concept_type(concept_type: &str) -> AppResult<()> {
    match concept_type {
        "deposit" | "withdrawal" | "transfer" => Ok(()),
        _ => Err(AppError {
            message: format!(
                "Tipo de concepto inválido: '{concept_type}'. Use deposit, withdrawal o transfer."
            ),
        }),
    }
}

pub async fn list_concepts_impl(
    db: &SqlitePool,
    account_id: i64,
    concept_type: Option<String>,
    active_only: bool,
) -> AppResult<Vec<Concept>> {
    if let Some(ref t) = concept_type {
        validate_concept_type(t)?;
    }

    let concepts = sqlx::query_as::<_, Concept>(&format!(
        "SELECT {CONCEPT_COLUMNS} FROM concepts
         WHERE account_id = ?
           AND (? IS NULL OR type = ?)
           AND (? = 0 OR is_active = 1)
         ORDER BY name COLLATE NOCASE ASC"
    ))
    .bind(account_id)
    .bind(&concept_type)
    .bind(&concept_type)
    .bind(if active_only { 1 } else { 0 })
    .fetch_all(db)
    .await?;

    Ok(concepts)
}

pub async fn create_concept_impl(
    db: &SqlitePool,
    account_id: i64,
    concept_type: String,
    name: String,
) -> AppResult<Concept> {
    validate_concept_type(&concept_type)?;

    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(AppError {
            message: "El nombre del concepto no puede estar vacío".to_string(),
        });
    }

    let now = local_timestamp();
    let concept = sqlx::query_as::<_, Concept>(&format!(
        "INSERT INTO concepts (account_id, type, name, is_active, created_at, updated_at)
         VALUES (?, ?, ?, 1, ?, ?)
         RETURNING {CONCEPT_COLUMNS}"
    ))
    .bind(account_id)
    .bind(&concept_type)
    .bind(&name)
    .bind(&now)
    .bind(&now)
    .fetch_one(db)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("UNIQUE") {
            AppError {
                message: format!("Ya existe un concepto '{name}' para este tipo de operación"),
            }
        } else {
            AppError::from(e)
        }
    })?;

    Ok(concept)
}

/// Crea el concepto si no existe (por account_id + type + name) y lo devuelve.
pub async fn ensure_concept_impl(
    db: &SqlitePool,
    account_id: i64,
    concept_type: String,
    name: String,
) -> AppResult<Concept> {
    validate_concept_type(&concept_type)?;

    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(AppError {
            message: "El nombre del concepto no puede estar vacío".to_string(),
        });
    }

    let now = local_timestamp();
    sqlx::query(
        "INSERT OR IGNORE INTO concepts (account_id, type, name, is_active, created_at, updated_at)
         VALUES (?, ?, ?, 1, ?, ?)",
    )
    .bind(account_id)
    .bind(&concept_type)
    .bind(&name)
    .bind(&now)
    .bind(&now)
    .execute(db)
    .await?;

    let concept = sqlx::query_as::<_, Concept>(&format!(
        "SELECT {CONCEPT_COLUMNS} FROM concepts
         WHERE account_id = ? AND type = ? AND name = ?"
    ))
    .bind(account_id)
    .bind(&concept_type)
    .bind(&name)
    .fetch_one(db)
    .await?;

    Ok(concept)
}

pub async fn get_concept_impl(db: &SqlitePool, id: i64) -> AppResult<Concept> {
    let concept = sqlx::query_as::<_, Concept>(&format!(
        "SELECT {CONCEPT_COLUMNS} FROM concepts WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(concept)
}

/// Valida que el concepto exista, pertenezca a la cuenta y coincida con el tipo de operación.
pub async fn validate_concept_for_operation(
    db: &SqlitePool,
    concept_id: i64,
    account_id: i64,
    operation_type: &str,
) -> AppResult<Concept> {
    let concept = get_concept_impl(db, concept_id).await.map_err(|_| AppError {
        message: format!("Concepto {concept_id} no encontrado"),
    })?;

    if concept.account_id != account_id {
        return Err(AppError {
            message: "El concepto no pertenece a esta cuenta".to_string(),
        });
    }

    if concept.r#type != operation_type {
        return Err(AppError {
            message: format!(
                "El concepto es de tipo '{}' y no se puede usar en una operación '{}'",
                concept.r#type, operation_type
            ),
        });
    }

    if !concept.is_active {
        return Err(AppError {
            message: "El concepto está inactivo".to_string(),
        });
    }

    Ok(concept)
}

pub async fn link_transaction_concept_impl(
    db: &SqlitePool,
    transaction_id: i64,
    concept_id: i64,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO transaction_concepts (transaction_id, concept_id) VALUES (?, ?)",
    )
    .bind(transaction_id)
    .bind(concept_id)
    .execute(db)
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn list_concepts(
    state: State<'_, AppState>,
    account_id: i64,
    concept_type: Option<String>,
    active_only: Option<bool>,
) -> AppResult<Vec<Concept>> {
    list_concepts_impl(
        &state.db,
        account_id,
        concept_type,
        active_only.unwrap_or(true),
    )
    .await
}

#[tauri::command]
pub async fn create_concept(
    state: State<'_, AppState>,
    account_id: i64,
    concept_type: String,
    name: String,
) -> AppResult<Concept> {
    create_concept_impl(&state.db, account_id, concept_type, name).await
}

#[tauri::command]
pub async fn ensure_concept(
    state: State<'_, AppState>,
    account_id: i64,
    concept_type: String,
    name: String,
) -> AppResult<Concept> {
    ensure_concept_impl(&state.db, account_id, concept_type, name).await
}
