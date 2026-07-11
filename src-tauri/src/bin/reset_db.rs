use pvsr_accounts_management_lib::db;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("failed to build tauri app context");

    let pool = db::init_db(app.handle()).await;

    reset(&pool).await;

    println!("Base de datos vaciada correctamente.");
}

async fn reset(pool: &SqlitePool) {
    // Primero transactions por la foreign key hacia accounts
    sqlx::query("DELETE FROM transactions")
        .execute(pool)
        .await
        .expect("failed to clear transactions");

    sqlx::query("DELETE FROM accounts")
        .execute(pool)
        .await
        .expect("failed to clear accounts");

    // Reinicia los contadores AUTOINCREMENT para que los próximos ids empiecen en 1
    sqlx::query("DELETE FROM sqlite_sequence WHERE name IN ('accounts', 'transactions')")
        .execute(pool)
        .await
        .expect("failed to reset autoincrement counters");
}
