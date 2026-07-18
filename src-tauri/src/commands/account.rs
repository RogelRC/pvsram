use crate::error::AppResult;
use crate::models::account::Account;
use crate::state::AppState;
use chrono::Local;
use sqlx::SqlitePool;
use tauri::State;
const ACCOUNT_COLUMNS: &str =
    "id, number, name, destination, comment, currency, color, created_at, updated_at";

fn local_timestamp() -> String {
    Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
pub async fn create_account_impl(
    db: &SqlitePool,
    number: String,
    name: String,
    destination: Option<String>,
    comment: String,
    currency: Option<String>,
    color: Option<String>,
) -> AppResult<Account> {
    let currency = currency.unwrap_or_else(|| "USD".to_string());
    let now = local_timestamp();
    let account = sqlx::query_as::<_, Account>(&format!(
        "INSERT INTO accounts (number, name, destination, comment, currency, color, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)
         RETURNING {ACCOUNT_COLUMNS}"
    ))
    .bind(number)
    .bind(name)
    .bind(destination)
    .bind(comment)
    .bind(currency)
    .bind(color)
    .bind(now.clone())
    .bind(now)
    .fetch_one(db)
    .await?;
    Ok(account)
}
pub async fn list_accounts_impl(db: &SqlitePool) -> AppResult<Vec<Account>> {
    let accounts = sqlx::query_as::<_, Account>(&format!(
        "SELECT {ACCOUNT_COLUMNS} FROM accounts ORDER BY id"
    ))
    .fetch_all(db)
    .await?;
    Ok(accounts)
}
pub async fn get_account_impl(db: &SqlitePool, id: i64) -> AppResult<Account> {
    let account = sqlx::query_as::<_, Account>(&format!(
        "SELECT {ACCOUNT_COLUMNS} FROM accounts WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(db)
    .await?;
    Ok(account)
}
pub async fn update_account_impl(
    db: &SqlitePool,
    id: i64,
    number: String,
    name: String,
    destination: Option<String>,
    comment: String,
    currency: String,
    color: Option<String>,
) -> AppResult<Account> {
    let now = local_timestamp();
    let account = sqlx::query_as::<_, Account>(&format!(
        "UPDATE accounts
         SET number = ?, name = ?, destination = ?, comment = ?, currency = ?, color = ?, updated_at = ?
         WHERE id = ?
         RETURNING {ACCOUNT_COLUMNS}"
    ))
    .bind(number)
    .bind(name)
    .bind(destination)
    .bind(comment)
    .bind(currency)
    .bind(color)
    .bind(now)
    .bind(id)
    .fetch_one(db)
    .await?;
    Ok(account)
}
pub async fn delete_account_impl(db: &SqlitePool, id: i64) -> AppResult<()> {
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
#[tauri::command]
pub async fn create_account(
    state: State<'_, AppState>,
    number: String,
    name: String,
    destination: Option<String>,
    comment: String,
    currency: Option<String>,
    color: Option<String>,
) -> AppResult<Account> {
    create_account_impl(
        &state.db,
        number,
        name,
        destination,
        comment,
        currency,
        color,
    )
    .await
}
#[tauri::command]
pub async fn list_accounts(state: State<'_, AppState>) -> AppResult<Vec<Account>> {
    list_accounts_impl(&state.db).await
}
#[tauri::command]
pub async fn get_account(state: State<'_, AppState>, id: i64) -> AppResult<Account> {
    get_account_impl(&state.db, id).await
}
#[tauri::command]
pub async fn update_account(
    state: State<'_, AppState>,
    id: i64,
    number: String,
    name: String,
    destination: Option<String>,
    comment: String,
    currency: String,
    color: Option<String>,
) -> AppResult<Account> {
    update_account_impl(
        &state.db,
        id,
        number,
        name,
        destination,
        comment,
        currency,
        color,
    )
    .await
}
#[tauri::command]
pub async fn delete_account(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    delete_account_impl(&state.db, id).await
}
