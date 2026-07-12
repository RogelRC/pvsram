use crate::error::AppResult;
use crate::models::transaction::Transaction;
use crate::state::AppState;
use sqlx::SqlitePool;
use sqlx::{QueryBuilder, Sqlite};
use tauri::State;
use chrono::{Datelike, Duration, Local, NaiveDate};


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

// ============ Reportes / Historial con filtros flexibles ============

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFilters {
    pub account_id: Option<i64>,
    pub related_account_id: Option<i64>,
    #[serde(rename = "type")]
    pub transaction_type: Option<String>, // "deposit" | "withdrawal" | "transfer"
    pub currency: Option<String>,
    pub amount_min: Option<f64>,
    pub amount_max: Option<f64>,
    pub description: Option<String>, // búsqueda parcial (LIKE)
    pub from: Option<String>,        // occurred_at >=
    pub to: Option<String>,          // occurred_at <=
    pub sort_by: Option<String>,     // "occurred_at" | "amount"
    pub sort_dir: Option<String>,    // "asc" | "desc"
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct TransactionRecord {
    pub id: i64,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub r#type: String,
    pub account_id: i64,
    pub related_account_id: Option<i64>,
    pub amount: f64,
    pub description: Option<String>,
    pub occurred_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub account_name: String,
    pub account_number: String,
    pub currency: String,
    pub related_account_name: Option<String>,
    pub related_account_number: Option<String>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct TransactionStats {
    pub total_count: i64,
    pub total_deposits: f64,
    pub total_withdrawals: f64,
    pub total_transfers: f64,
}

/// Aplica los filtros comunes a un QueryBuilder que ya seleccionó
/// desde `transactions t JOIN accounts a ... LEFT JOIN accounts ra ...`
fn apply_filters(qb: &mut QueryBuilder<Sqlite>, f: &TransactionFilters) {
    let mut has_where = false;

    if let Some(account_id) = f.account_id {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("(t.account_id = ")
            .push_bind(account_id)
            .push(" OR t.related_account_id = ")
            .push_bind(account_id)
            .push(")");
    }

    if let Some(related_account_id) = f.related_account_id {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("t.related_account_id = ")
            .push_bind(related_account_id);
    }

    if let Some(ref t) = f.transaction_type {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("t.type = ").push_bind(t.clone());
    }

    if let Some(ref currency) = f.currency {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("a.currency = ").push_bind(currency.clone());
    }

    if let Some(amount_min) = f.amount_min {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("t.amount >= ").push_bind(amount_min);
    }

    if let Some(amount_max) = f.amount_max {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("t.amount <= ").push_bind(amount_max);
    }

    if let Some(ref desc) = f.description {
        if !desc.trim().is_empty() {
            qb.push(if has_where { " AND " } else { " WHERE " });
            has_where = true;
            qb.push("t.description LIKE ")
                .push_bind(format!("%{}%", desc.trim()));
        }
    }

    if let Some(ref from) = f.from {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("t.occurred_at >= ").push_bind(from.clone());
    }

    if let Some(ref to) = f.to {
        qb.push(if has_where { " AND " } else { " WHERE " });
        has_where = true;
        qb.push("t.occurred_at <= ").push_bind(to.clone());
    }

    let _ = has_where;
}

const JOINED_FROM: &str = "FROM transactions t
    JOIN accounts a ON a.id = t.account_id
    LEFT JOIN accounts ra ON ra.id = t.related_account_id";

pub async fn list_transactions_report_impl(
    db: &SqlitePool,
    filters: &TransactionFilters,
) -> AppResult<Vec<TransactionRecord>> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT
            t.id, t.type, t.account_id, t.related_account_id, t.amount, t.description,
            t.occurred_at, t.created_at, t.updated_at,
            a.name AS account_name, a.number AS account_number, a.currency AS currency,
            ra.name AS related_account_name, ra.number AS related_account_number
         {JOINED_FROM}"
    ));

    apply_filters(&mut qb, filters);

    let sort_column = match filters.sort_by.as_deref() {
        Some("amount") => "t.amount",
        _ => "t.occurred_at",
    };
    let sort_dir = match filters.sort_dir.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };
    qb.push(format!(
        " ORDER BY {sort_column} {sort_dir}, t.id {sort_dir}"
    ));

    let limit = filters.limit.unwrap_or(50).clamp(1, 500);
    let offset = filters.offset.unwrap_or(0).max(0);
    qb.push(" LIMIT ").push_bind(limit);
    qb.push(" OFFSET ").push_bind(offset);

    let rows = qb
        .build_query_as::<TransactionRecord>()
        .fetch_all(db)
        .await?;

    Ok(rows)
}

pub async fn count_transactions_report_impl(
    db: &SqlitePool,
    filters: &TransactionFilters,
) -> AppResult<i64> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(format!("SELECT COUNT(*) {JOINED_FROM}"));

    apply_filters(&mut qb, filters);

    let count: (i64,) = qb.build_query_as().fetch_one(db).await?;
    Ok(count.0)
}

pub async fn get_transactions_stats_impl(
    db: &SqlitePool,
    filters: &TransactionFilters,
) -> AppResult<TransactionStats> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(format!(
        "SELECT
            COUNT(*) AS total_count,
            COALESCE(SUM(CASE WHEN t.type = 'deposit' THEN t.amount ELSE 0 END), 0) AS total_deposits,
            COALESCE(SUM(CASE WHEN t.type = 'withdrawal' THEN t.amount ELSE 0 END), 0) AS total_withdrawals,
            COALESCE(SUM(CASE WHEN t.type = 'transfer' THEN t.amount ELSE 0 END), 0) AS total_transfers
         {JOINED_FROM}"
    ));

    apply_filters(&mut qb, filters);

    let row = qb
        .build_query_as::<TransactionStats>()
        .fetch_one(db)
        .await?;

    Ok(row)
}

#[tauri::command]
pub async fn list_transactions_report(
    state: State<'_, AppState>,
    filters: TransactionFilters,
) -> AppResult<Vec<TransactionRecord>> {
    list_transactions_report_impl(&state.db, &filters).await
}

#[tauri::command]
pub async fn count_transactions_report(
    state: State<'_, AppState>,
    filters: TransactionFilters,
) -> AppResult<i64> {
    count_transactions_report_impl(&state.db, &filters).await
}

#[tauri::command]
pub async fn get_transactions_stats(
    state: State<'_, AppState>,
    filters: TransactionFilters,
) -> AppResult<TransactionStats> {
    get_transactions_stats_impl(&state.db, &filters).await
}

// ============ Estadísticas por cuenta y por moneda ============

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsFilters {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct AccountStatsRow {
    pub account_id: i64,
    pub account_name: String,
    pub account_number: String,
    pub currency: String,
    pub color: Option<String>,
    pub total_deposits: f64,
    pub total_withdrawals: f64,
    pub total_transfers_out: f64,
    pub total_transfers_in: f64,
    pub transaction_count: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct AccountStats {
    pub account_id: i64,
    pub account_name: String,
    pub account_number: String,
    pub currency: String,
    pub color: Option<String>,
    pub total_deposits: f64,
    pub total_withdrawals: f64,
    pub total_transfers_out: f64,
    pub total_transfers_in: f64,
    pub balance: f64,
    pub transaction_count: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct CurrencyStats {
    pub currency: String,
    pub account_count: i64,
    pub total_balance: f64,
    pub total_deposits: f64,
    pub total_withdrawals: f64,
    pub total_transferred: f64,
    pub transaction_count: i64,
}

pub async fn get_stats_by_account_impl(
    db: &SqlitePool,
    filters: &StatsFilters,
) -> AppResult<Vec<AccountStats>> {
    let rows = sqlx::query_as::<_, AccountStatsRow>(
        "SELECT
            a.id AS account_id,
            a.name AS account_name,
            a.number AS account_number,
            a.currency AS currency,
            a.color AS color,
            COALESCE(SUM(CASE
                WHEN t.type = 'deposit' AND t.account_id = a.id THEN t.amount
                ELSE 0.0
            END), 0.0) AS total_deposits,
            COALESCE(SUM(CASE
                WHEN t.type = 'withdrawal' AND t.account_id = a.id THEN t.amount
                ELSE 0.0
            END), 0.0) AS total_withdrawals,
            COALESCE(SUM(CASE
                WHEN t.type = 'transfer' AND t.account_id = a.id THEN t.amount
                ELSE 0.0
            END), 0.0) AS total_transfers_out,
            COALESCE(SUM(CASE
                WHEN t.type = 'transfer' AND t.related_account_id = a.id THEN t.amount
                ELSE 0.0
            END), 0.0) AS total_transfers_in,
            COUNT(CASE
                WHEN t.account_id = a.id OR t.related_account_id = a.id THEN t.id
            END) AS transaction_count
         FROM accounts a
         LEFT JOIN transactions t
            ON (t.account_id = a.id OR t.related_account_id = a.id)
           AND (? IS NULL OR t.occurred_at >= ?)
           AND (? IS NULL OR t.occurred_at <= ?)
         GROUP BY a.id
         ORDER BY a.id",
    )
    .bind(&filters.from)
    .bind(&filters.from)
    .bind(&filters.to)
    .bind(&filters.to)
    .fetch_all(db)
    .await?;

    let stats = rows
        .into_iter()
        .map(|r| {
            let balance = r.total_deposits + r.total_transfers_in
                - r.total_withdrawals
                - r.total_transfers_out;
            AccountStats {
                account_id: r.account_id,
                account_name: r.account_name,
                account_number: r.account_number,
                currency: r.currency,
                color: r.color,
                total_deposits: r.total_deposits,
                total_withdrawals: r.total_withdrawals,
                total_transfers_out: r.total_transfers_out,
                total_transfers_in: r.total_transfers_in,
                balance,
                transaction_count: r.transaction_count,
            }
        })
        .collect();

    Ok(stats)
}

pub async fn get_stats_by_currency_impl(
    db: &SqlitePool,
    filters: &StatsFilters,
) -> AppResult<Vec<CurrencyStats>> {
    let account_stats = get_stats_by_account_impl(db, filters).await?;

    let mut by_currency: std::collections::BTreeMap<String, CurrencyStats> =
        std::collections::BTreeMap::new();

    for s in account_stats {
        let entry = by_currency
            .entry(s.currency.clone())
            .or_insert_with(|| CurrencyStats {
                currency: s.currency.clone(),
                account_count: 0,
                total_balance: 0.0,
                total_deposits: 0.0,
                total_withdrawals: 0.0,
                total_transferred: 0.0,
                transaction_count: 0,
            });

        entry.account_count += 1;
        entry.total_balance += s.balance;
        entry.total_deposits += s.total_deposits;
        entry.total_withdrawals += s.total_withdrawals;
        entry.total_transferred += s.total_transfers_out;
        entry.transaction_count += s.transaction_count;
    }

    Ok(by_currency.into_values().collect())
}

#[tauri::command]
pub async fn get_stats_by_account(
    state: State<'_, AppState>,
    filters: StatsFilters,
) -> AppResult<Vec<AccountStats>> {
    get_stats_by_account_impl(&state.db, &filters).await
}

#[tauri::command]
pub async fn get_stats_by_currency(
    state: State<'_, AppState>,
    filters: StatsFilters,
) -> AppResult<Vec<CurrencyStats>> {
    get_stats_by_currency_impl(&state.db, &filters).await
}


// ============ Informes ============

fn month_first_last(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
    let start = NaiveDate::from_ymd_opt(year, month, 1).expect("fecha inválida");
    let last = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    } - Duration::days(1);
    (start, last)
}

fn current_year_month() -> (i32, u32) {
    let today = Local::now().naive_local().date();
    (today.year(), today.month())
}

// --- Informe 2: entradas y salidas en un rango de tiempo ---

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovementsFilters {
    pub from: Option<String>, // "YYYY-MM-DD"
    pub to: Option<String>,   // "YYYY-MM-DD"
}

fn resolve_movements_range(filters: &MovementsFilters) -> (String, String) {
    match (&filters.from, &filters.to) {
        (Some(from), Some(to)) if !from.is_empty() && !to.is_empty() => {
            (format!("{from} 00:00:00"), format!("{to} 23:59:59"))
        }
        _ => {
            let (year, month) = current_year_month();
            let (start, end) = month_first_last(year, month);
            (format!("{start} 00:00:00"), format!("{end} 23:59:59"))
        }
    }
}

#[tauri::command]
pub async fn get_movements_report(
    state: State<'_, AppState>,
    filters: MovementsFilters,
) -> AppResult<Vec<AccountStats>> {
    let (from, to) = resolve_movements_range(&filters);
    get_stats_by_account_impl(
        &state.db,
        &StatsFilters {
            from: Some(from),
            to: Some(to),
        },
    )
    .await
}

// --- Informe 3: saldo inicial y final del mes ---

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyBalanceFilters {
    pub year: Option<i32>,
    pub month: Option<u32>, // 1-12
}

#[derive(Debug, sqlx::FromRow)]
struct MonthlyBalanceRow {
    account_id: i64,
    account_name: String,
    account_number: String,
    currency: String,
    color: Option<String>,
    opening_balance: f64,
    closing_balance: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct MonthlyBalanceReport {
    pub account_id: i64,
    pub account_name: String,
    pub account_number: String,
    pub currency: String,
    pub color: Option<String>,
    pub opening_balance: f64,
    pub closing_balance: f64,
    pub net_change: f64,
    pub period_start: String,
    pub period_end: String,
    pub month_finished: bool,
}

pub async fn get_monthly_balance_report_impl(
    db: &SqlitePool,
    filters: &MonthlyBalanceFilters,
) -> AppResult<Vec<MonthlyBalanceReport>> {
    let (year, month) = match (filters.year, filters.month) {
        (Some(y), Some(m)) => (y, m),
        _ => current_year_month(),
    };
    let (month_start, month_end) = month_first_last(year, month);

    let today = Local::now().naive_local().date();
    let is_current_month = today.year() == year && today.month() == month;
    let effective_end = if is_current_month && today < month_end {
        today
    } else {
        month_end
    };
    let month_finished = !(is_current_month && today < month_end);

    let start = format!("{month_start} 00:00:00");
    let end = format!("{effective_end} 23:59:59");

    let rows = sqlx::query_as::<_, MonthlyBalanceRow>(
        "SELECT
            a.id AS account_id,
            a.name AS account_name,
            a.number AS account_number,
            a.currency AS currency,
            a.color AS color,
            COALESCE(SUM(CASE
                WHEN t.occurred_at < ? AND t.type = 'deposit' AND t.account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0)
            + COALESCE(SUM(CASE
                WHEN t.occurred_at < ? AND t.type = 'transfer' AND t.related_account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0)
            - COALESCE(SUM(CASE
                WHEN t.occurred_at < ? AND t.type = 'withdrawal' AND t.account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0)
            - COALESCE(SUM(CASE
                WHEN t.occurred_at < ? AND t.type = 'transfer' AND t.account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0) AS opening_balance,
            COALESCE(SUM(CASE
                WHEN t.occurred_at <= ? AND t.type = 'deposit' AND t.account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0)
            + COALESCE(SUM(CASE
                WHEN t.occurred_at <= ? AND t.type = 'transfer' AND t.related_account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0)
            - COALESCE(SUM(CASE
                WHEN t.occurred_at <= ? AND t.type = 'withdrawal' AND t.account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0)
            - COALESCE(SUM(CASE
                WHEN t.occurred_at <= ? AND t.type = 'transfer' AND t.account_id = a.id
                THEN t.amount ELSE 0.0 END), 0.0) AS closing_balance
         FROM accounts a
         LEFT JOIN transactions t
            ON (t.account_id = a.id OR t.related_account_id = a.id)
         GROUP BY a.id
         ORDER BY a.id",
    )
    .bind(&start)
    .bind(&start)
    .bind(&start)
    .bind(&start)
    .bind(&end)
    .bind(&end)
    .bind(&end)
    .bind(&end)
    .fetch_all(db)
    .await?;

    let reports = rows
        .into_iter()
        .map(|r| MonthlyBalanceReport {
            account_id: r.account_id,
            account_name: r.account_name,
            account_number: r.account_number,
            currency: r.currency,
            color: r.color,
            opening_balance: r.opening_balance,
            closing_balance: r.closing_balance,
            net_change: r.closing_balance - r.opening_balance,
            period_start: month_start.to_string(),
            period_end: effective_end.to_string(),
            month_finished,
        })
        .collect();

    Ok(reports)
}

#[tauri::command]
pub async fn get_monthly_balance_report(
    state: State<'_, AppState>,
    filters: MonthlyBalanceFilters,
) -> AppResult<Vec<MonthlyBalanceReport>> {
    get_monthly_balance_report_impl(&state.db, &filters).await
}
