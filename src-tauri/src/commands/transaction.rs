use crate::error::AppResult;
use crate::models::transaction::Transaction;
use crate::state::AppState;
use sqlx::SqlitePool;
use tauri::State;

const TRANSACTION_COLUMNS: &str =
    "id, type, account_id, related_account_id, amount, description, occurred_at, created_at, updated_at";

pub async fn create_deposit_impl(
    db: &SqlitePool,
    account_id: i64,
    amount: f64,
    description: Option<String>,
    occurred_at: Option<String>,
) -> AppResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "INSERT INTO transactions (type, account_id, amount, description, occurred_at)
         VALUES ('deposit', ?, ?, ?, COALESCE(?, CURRENT_TIMESTAMP))
         RETURNING {TRANSACTION_COLUMNS}"
    ))
    .bind(account_id)
    .bind(amount)
    .bind(description)
    .bind(occurred_at)
    .fetch_one(db)
    .await?;

    Ok(transaction)
}

pub async fn create_withdrawal_impl(
    db: &SqlitePool,
    account_id: i64,
    amount: f64,
    description: Option<String>,
    occurred_at: Option<String>,
) -> AppResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "INSERT INTO transactions (type, account_id, amount, description, occurred_at)
         VALUES ('withdrawal', ?, ?, ?, COALESCE(?, CURRENT_TIMESTAMP))
         RETURNING {TRANSACTION_COLUMNS}"
    ))
    .bind(account_id)
    .bind(amount)
    .bind(description)
    .bind(occurred_at)
    .fetch_one(db)
    .await?;

    Ok(transaction)
}

pub async fn create_transfer_impl(
    db: &SqlitePool,
    account_id: i64,
    related_account_id: i64,
    amount: f64,
    description: Option<String>,
    occurred_at: Option<String>,
) -> AppResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "INSERT INTO transactions (type, account_id, related_account_id, amount, description, occurred_at)
         VALUES ('transfer', ?, ?, ?, ?, COALESCE(?, CURRENT_TIMESTAMP))
         RETURNING {TRANSACTION_COLUMNS}"
    ))
    .bind(account_id)
    .bind(related_account_id)
    .bind(amount)
    .bind(description)
    .bind(occurred_at)
    .fetch_one(db)
    .await?;

    Ok(transaction)
}

pub async fn list_transactions_impl(
    db: &SqlitePool,
    account_id: Option<i64>,
    from: Option<String>,
    to: Option<String>,
) -> AppResult<Vec<Transaction>> {
    let transactions = sqlx::query_as::<_, Transaction>(&format!(
        "SELECT {TRANSACTION_COLUMNS} FROM transactions
         WHERE (?1 IS NULL OR account_id = ?1 OR related_account_id = ?1)
           AND (?2 IS NULL OR occurred_at >= ?2)
           AND (?3 IS NULL OR occurred_at <= ?3)
         ORDER BY occurred_at DESC"
    ))
    .bind(account_id)
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    Ok(transactions)
}

pub async fn get_transaction_impl(db: &SqlitePool, id: i64) -> AppResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>(&format!(
        "SELECT {TRANSACTION_COLUMNS} FROM transactions WHERE id = ?"
    ))
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(transaction)
}

pub async fn delete_transaction_impl(db: &SqlitePool, id: i64) -> AppResult<()> {
    sqlx::query("DELETE FROM transactions WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn get_account_balance_impl(
    db: &SqlitePool,
    account_id: i64,
    as_of: Option<String>,
) -> AppResult<f64> {
    let balance: (f64,) = sqlx::query_as(
        "SELECT
            COALESCE(SUM(CASE
                WHEN type = 'deposit' AND account_id = ?1 THEN amount
                WHEN type = 'transfer' AND related_account_id = ?1 THEN amount
                ELSE 0
            END), 0)
            -
            COALESCE(SUM(CASE
                WHEN type = 'withdrawal' AND account_id = ?1 THEN amount
                WHEN type = 'transfer' AND account_id = ?1 THEN amount
                ELSE 0
            END), 0) AS balance
         FROM transactions
         WHERE (account_id = ?1 OR related_account_id = ?1)
           AND (?2 IS NULL OR occurred_at <= ?2)",
    )
    .bind(account_id)
    .bind(as_of)
    .fetch_one(db)
    .await?;

    Ok(balance.0)
}

#[tauri::command]
pub async fn create_deposit(
    state: State<'_, AppState>,
    account_id: i64,
    amount: f64,
    description: Option<String>,
    occurred_at: Option<String>,
) -> AppResult<Transaction> {
    create_deposit_impl(&state.db, account_id, amount, description, occurred_at).await
}

#[tauri::command]
pub async fn create_withdrawal(
    state: State<'_, AppState>,
    account_id: i64,
    amount: f64,
    description: Option<String>,
    occurred_at: Option<String>,
) -> AppResult<Transaction> {
    create_withdrawal_impl(&state.db, account_id, amount, description, occurred_at).await
}

#[tauri::command]
pub async fn create_transfer(
    state: State<'_, AppState>,
    account_id: i64,
    related_account_id: i64,
    amount: f64,
    description: Option<String>,
    occurred_at: Option<String>,
) -> AppResult<Transaction> {
    create_transfer_impl(
        &state.db,
        account_id,
        related_account_id,
        amount,
        description,
        occurred_at,
    )
    .await
}

#[tauri::command]
pub async fn list_transactions(
    state: State<'_, AppState>,
    account_id: Option<i64>,
    from: Option<String>,
    to: Option<String>,
) -> AppResult<Vec<Transaction>> {
    list_transactions_impl(&state.db, account_id, from, to).await
}

#[tauri::command]
pub async fn get_transaction(state: State<'_, AppState>, id: i64) -> AppResult<Transaction> {
    get_transaction_impl(&state.db, id).await
}

#[tauri::command]
pub async fn delete_transaction(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    delete_transaction_impl(&state.db, id).await
}

#[tauri::command]
pub async fn get_account_balance(
    state: State<'_, AppState>,
    account_id: i64,
    as_of: Option<String>,
) -> AppResult<f64> {
    get_account_balance_impl(&state.db, account_id, as_of).await
}
