use pvsr_accounts_management_lib::commands::account::{
    create_account_impl, delete_account_impl, get_account_impl, list_accounts_impl,
    update_account_impl,
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
#[tokio::test]
async fn test_create_account() {
    let db = setup_db().await;
    let account = create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta principal".to_string(),
        None,
        "Rogel".to_string(),
        Some("USD".to_string()),
        Some("#FF0000".to_string()),
    )
    .await
    .expect("create_account failed");
    assert_eq!(account.number, "ACC-001");
    assert_eq!(account.name, "Cuenta principal");
    assert_eq!(account.owner, "Rogel");
    assert_eq!(account.currency, "USD");
    assert_eq!(account.color, Some("#FF0000".to_string()));
}
#[tokio::test]
async fn test_list_accounts() {
    let db = setup_db().await;
    create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta A".to_string(),
        None,
        "Rogel".to_string(),
        None,
        None,
    )
    .await
    .unwrap();
    create_account_impl(
        &db,
        "ACC-002".to_string(),
        "Cuenta B".to_string(),
        None,
        "Rogel".to_string(),
        None,
        None,
    )
    .await
    .unwrap();
    let accounts = list_accounts_impl(&db).await.expect("list_accounts failed");
    assert_eq!(accounts.len(), 2);
}
#[tokio::test]
async fn test_get_account() {
    let db = setup_db().await;
    let created = create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta principal".to_string(),
        None,
        "Rogel".to_string(),
        None,
        None,
    )
    .await
    .unwrap();
    let fetched = get_account_impl(&db, created.id)
        .await
        .expect("get_account failed");
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.number, "ACC-001");
}
#[tokio::test]
async fn test_update_account() {
    let db = setup_db().await;
    let created = create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta original".to_string(),
        None,
        "Rogel".to_string(),
        Some("USD".to_string()),
        None,
    )
    .await
    .unwrap();
    let updated = update_account_impl(
        &db,
        created.id,
        "ACC-001".to_string(),
        "Cuenta actualizada".to_string(),
        Some("Nueva descripción".to_string()),
        "Rogel".to_string(),
        "EUR".to_string(),
        Some("#00FF00".to_string()),
    )
    .await
    .expect("update_account failed");
    assert_eq!(updated.name, "Cuenta actualizada");
    assert_eq!(updated.currency, "EUR");
    assert_eq!(updated.color, Some("#00FF00".to_string()));
}
#[tokio::test]
async fn test_delete_account() {
    let db = setup_db().await;
    let created = create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta a borrar".to_string(),
        None,
        "Rogel".to_string(),
        None,
        None,
    )
    .await
    .unwrap();
    delete_account_impl(&db, created.id)
        .await
        .expect("delete_account failed");
    let accounts = list_accounts_impl(&db).await.unwrap();
    assert_eq!(accounts.len(), 0);
}
#[tokio::test]
async fn test_create_account_duplicate_number_fails() {
    let db = setup_db().await;
    create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta A".to_string(),
        None,
        "Rogel".to_string(),
        None,
        None,
    )
    .await
    .unwrap();
    let result = create_account_impl(
        &db,
        "ACC-001".to_string(),
        "Cuenta B".to_string(),
        None,
        "Rogel".to_string(),
        None,
        None,
    )
    .await;
    assert!(result.is_err());
}
