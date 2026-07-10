use pvsr_accounts_management_lib::commands::account::create_account_impl;
use pvsr_accounts_management_lib::commands::transaction::{
    create_deposit_impl, create_transfer_impl, create_withdrawal_impl, delete_transaction_impl,
    get_account_balance_impl, get_transaction_impl, list_transactions_impl,
};
use sqlx::sqlite::SqlitePoolOptions;

async fn setup_db() -> sqlx::SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("failed to connect to in-memory db");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");
    pool
}

async fn create_test_account(db: &sqlx::SqlitePool, number: &str) -> i64 {
    create_account_impl(
        db,
        number.to_string(),
        format!("Cuenta {number}"),
        None,
        "Rogel".to_string(),
        Some("USD".to_string()),
        None,
    )
    .await
    .expect("failed to create test account")
    .id
}

#[tokio::test]
async fn test_create_deposit() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    let transaction =
        create_deposit_impl(&db, account_id, 100.0, Some("Salario".to_string()), None)
            .await
            .expect("create_deposit failed");

    assert_eq!(transaction.r#type, "deposit");
    assert_eq!(transaction.account_id, account_id);
    assert_eq!(transaction.related_account_id, None);
    assert_eq!(transaction.amount, 100.0);
}

#[tokio::test]
async fn test_create_withdrawal() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    let transaction =
        create_withdrawal_impl(&db, account_id, 30.0, Some("Compras".to_string()), None)
            .await
            .expect("create_withdrawal failed");

    assert_eq!(transaction.r#type, "withdrawal");
    assert_eq!(transaction.account_id, account_id);
    assert_eq!(transaction.amount, 30.0);
}

#[tokio::test]
async fn test_create_transfer() {
    let db = setup_db().await;
    let account_a = create_test_account(&db, "ACC-001").await;
    let account_b = create_test_account(&db, "ACC-002").await;

    let transaction = create_transfer_impl(&db, account_a, account_b, 50.0, None, None)
        .await
        .expect("create_transfer failed");

    assert_eq!(transaction.r#type, "transfer");
    assert_eq!(transaction.account_id, account_a);
    assert_eq!(transaction.related_account_id, Some(account_b));
    assert_eq!(transaction.amount, 50.0);
}

#[tokio::test]
async fn test_transfer_to_same_account_fails() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    let result = create_transfer_impl(&db, account_id, account_id, 50.0, None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_transaction() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    let created = create_deposit_impl(&db, account_id, 100.0, None, None)
        .await
        .unwrap();

    let fetched = get_transaction_impl(&db, created.id)
        .await
        .expect("get_transaction failed");

    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.amount, 100.0);
}

#[tokio::test]
async fn test_delete_transaction() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    let created = create_deposit_impl(&db, account_id, 100.0, None, None)
        .await
        .unwrap();

    delete_transaction_impl(&db, created.id)
        .await
        .expect("delete_transaction failed");

    let result = get_transaction_impl(&db, created.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_transactions_filtered_by_account() {
    let db = setup_db().await;
    let account_a = create_test_account(&db, "ACC-001").await;
    let account_b = create_test_account(&db, "ACC-002").await;

    create_deposit_impl(&db, account_a, 100.0, None, None)
        .await
        .unwrap();
    create_deposit_impl(&db, account_b, 200.0, None, None)
        .await
        .unwrap();

    let transactions_a = list_transactions_impl(&db, Some(account_a), None, None)
        .await
        .expect("list_transactions failed");

    assert_eq!(transactions_a.len(), 1);
    assert_eq!(transactions_a[0].account_id, account_a);
}

#[tokio::test]
async fn test_list_transactions_includes_transfers_for_related_account() {
    let db = setup_db().await;
    let account_a = create_test_account(&db, "ACC-001").await;
    let account_b = create_test_account(&db, "ACC-002").await;

    create_transfer_impl(&db, account_a, account_b, 50.0, None, None)
        .await
        .unwrap();

    let transactions_b = list_transactions_impl(&db, Some(account_b), None, None)
        .await
        .expect("list_transactions failed");

    assert_eq!(transactions_b.len(), 1);
    assert_eq!(transactions_b[0].r#type, "transfer");
}

#[tokio::test]
async fn test_list_transactions_filtered_by_date_range() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    create_deposit_impl(
        &db,
        account_id,
        100.0,
        None,
        Some("2025-01-01T00:00:00Z".to_string()),
    )
    .await
    .unwrap();

    create_deposit_impl(
        &db,
        account_id,
        200.0,
        None,
        Some("2025-06-01T00:00:00Z".to_string()),
    )
    .await
    .unwrap();

    let transactions = list_transactions_impl(
        &db,
        Some(account_id),
        Some("2025-05-01T00:00:00Z".to_string()),
        Some("2025-12-31T00:00:00Z".to_string()),
    )
    .await
    .expect("list_transactions failed");

    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].amount, 200.0);
}

#[tokio::test]
async fn test_balance_deposit_and_withdrawal() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    create_deposit_impl(&db, account_id, 100.0, None, None)
        .await
        .unwrap();
    create_withdrawal_impl(&db, account_id, 30.0, None, None)
        .await
        .unwrap();

    let balance = get_account_balance_impl(&db, account_id, None)
        .await
        .expect("get_account_balance failed");

    assert_eq!(balance, 70.0);
}

#[tokio::test]
async fn test_balance_with_transfer_between_own_accounts() {
    let db = setup_db().await;
    let account_a = create_test_account(&db, "ACC-001").await;
    let account_b = create_test_account(&db, "ACC-002").await;

    create_deposit_impl(&db, account_a, 100.0, None, None)
        .await
        .unwrap();
    create_transfer_impl(&db, account_a, account_b, 40.0, None, None)
        .await
        .unwrap();

    let balance_a = get_account_balance_impl(&db, account_a, None)
        .await
        .unwrap();
    let balance_b = get_account_balance_impl(&db, account_b, None)
        .await
        .unwrap();

    assert_eq!(balance_a, 60.0);
    assert_eq!(balance_b, 40.0);
}

#[tokio::test]
async fn test_balance_as_of_specific_date() {
    let db = setup_db().await;
    let account_id = create_test_account(&db, "ACC-001").await;

    create_deposit_impl(
        &db,
        account_id,
        100.0,
        None,
        Some("2025-01-01T00:00:00Z".to_string()),
    )
    .await
    .unwrap();

    create_deposit_impl(
        &db,
        account_id,
        200.0,
        None,
        Some("2025-06-01T00:00:00Z".to_string()),
    )
    .await
    .unwrap();

    let balance_before_june =
        get_account_balance_impl(&db, account_id, Some("2025-03-01T00:00:00Z".to_string()))
            .await
            .unwrap();

    let balance_after_june =
        get_account_balance_impl(&db, account_id, Some("2025-12-31T00:00:00Z".to_string()))
            .await
            .unwrap();

    assert_eq!(balance_before_june, 100.0);
    assert_eq!(balance_after_june, 300.0);
}
