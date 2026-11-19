use pvsr_accounts_management_lib::commands::seed::{seed_dummy_data_impl, SeedDummyOptions};
use pvsr_accounts_management_lib::db;

#[tokio::main]
async fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("failed to build tauri app context");

    let pool = db::init_db(app.handle()).await;

    let result = seed_dummy_data_impl(
        &pool,
        SeedDummyOptions {
            accounts: Some(200),
            transactions: Some(5_000),
            clear_existing: Some(true),
        },
    )
    .await
    .expect("failed to seed dummy data");

    println!(
        "Seed completado: {} cuentas, {} conceptos, {} transacciones, {} vínculos tx-concepto (limpieza previa: {})",
        result.accounts_created,
        result.concepts_created,
        result.transactions_created,
        result.transaction_concepts_created,
        result.cleared_existing
    );
}
