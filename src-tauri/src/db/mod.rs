use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Sqlite;
use tauri::Manager;

pub async fn init_db(app_handle: &tauri::AppHandle) -> SqlitePool {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir");

    std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");

    let db_path = app_dir.join("pvsr_accounts.db");
    let db_url = format!("sqlite:{}", db_path.display());

    // Crear base de datos si no existe
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url)
            .await
            .expect("failed to create database");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to database");

    // Migraciones con mejor manejo de errores
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("✅ Migraciones aplicadas correctamente"),
        Err(e) => {
            eprintln!("❌ Error al ejecutar las migraciones: {}", e);
            // Opcional: puedes hacer panic aquí si quieres que la app falle
            // panic!("Migration failed: {}", e);

            // O dejar que la app continúe (depende de tu caso)
        }
    }

    pool
}
