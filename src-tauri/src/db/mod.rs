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

    // Migraciones con mejor manejo de errores. Si una migración ya aplicada
    // cambió su checksum (p. ej. por finales de línea/encoding entre builds),
    // reparamos el checksum registrado y reintentamos una vez.
    let migrator = sqlx::migrate!("./migrations");
    if let Err(e) = migrator.run(&pool).await {
        eprintln!("⚠️ Error al ejecutar las migraciones: {e}. Intentando reparar checksums...");
        if let Err(repair_err) = repair_migration_checksums(&pool, &migrator).await {
            eprintln!("❌ No se pudieron reparar checksums: {repair_err}");
            panic!("Error en migraciones: {e}");
        }
        migrator.run(&pool).await.unwrap_or_else(|e2| {
            eprintln!("❌ Error al ejecutar las migraciones tras reparar: {e2}");
            panic!("Error en migraciones: {e2}");
        });
        println!("✅ Migraciones aplicadas correctamente tras reparar checksums");
    } else {
        println!("✅ Migraciones aplicadas correctamente");
    }

    pool
}

async fn repair_migration_checksums(
    pool: &SqlitePool,
    migrator: &sqlx::migrate::Migrator,
) -> Result<(), sqlx::Error> {
    for migration in migrator.iter() {
        let checksum: Vec<u8> = migration.checksum.as_ref().to_vec();
        sqlx::query(
            "UPDATE _sqlx_migrations SET checksum = ? WHERE version = ? AND checksum != ?",
        )
        .bind(&checksum)
        .bind(migration.version as i64)
        .bind(&checksum)
        .execute(pool)
        .await?;
    }
    Ok(())
}
