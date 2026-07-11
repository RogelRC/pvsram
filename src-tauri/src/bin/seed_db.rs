use pvsr_accounts_management_lib::db;
use rand::Rng;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("failed to build tauri app context");

    let pool = db::init_db(app.handle()).await;
    seed(&pool).await;
    println!("✅ Seed completado: 100 cuentas + 1000 transacciones");
}

async fn seed(pool: &SqlitePool) {
    let mut rng = rand::thread_rng();
    println!("Generando 100 cuentas...");

    let mut account_ids: Vec<i64> = Vec::with_capacity(100);

    let account_names = [
        "Principal",
        "Ahorros",
        "Inversiones",
        "Personal",
        "Empresa",
        "Viajes",
        "Emergencia",
        "Negocio",
        "Educación",
        "Salud",
    ];

    let owners = [
        "Juan Pérez",
        "María López",
        "Carlos Rodríguez",
        "Ana Martínez",
        "Roberto Gómez",
        "Laura Sánchez",
        "Diego Morales",
        "Sofía Ramírez",
    ];

    let currencies = ["USD", "EUR", "MXN", "ARS"];
    let colors = [
        "#3b82f6", "#10b981", "#f59e0b", "#ef4444", "#8b5cf6", "#ec4899", "#14b8a6",
    ];

    // Crear 100 cuentas
    for i in 1..=100 {
        let number = format!("ACC-{:04}", i);
        let base_name = account_names[rng.gen_range(0..account_names.len())];
        let name = format!("{} {}", base_name, rng.gen_range(1..=99));
        let owner = owners[rng.gen_range(0..owners.len())].to_string();
        let currency = currencies[rng.gen_range(0..currencies.len())].to_string();
        let color = colors[rng.gen_range(0..colors.len())].to_string();

        let description = if rng.gen_bool(0.65) {
            Some(format!("Cuenta de prueba {}", i))
        } else {
            None
        };

        let (id,): (i64,) = sqlx::query_as(
            r#"
            INSERT INTO accounts (number, name, description, owner, currency, color)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
        )
        .bind(&number)
        .bind(&name)
        .bind(description)
        .bind(&owner)
        .bind(&currency)
        .bind(&color)
        .fetch_one(pool)
        .await
        .unwrap_or_else(|e| panic!("Failed to insert account {}: {}", i, e));

        account_ids.push(id);
    }

    println!("✅ 100 cuentas creadas. Generando 1000 transacciones...");

    // Crear 1000 transacciones aleatorias
    for _ in 0..1000 {
        let from_idx = rng.gen_range(0..account_ids.len());
        let account_id = account_ids[from_idx];
        let is_transfer = rng.gen_bool(0.7);

        let related_account_id = if is_transfer {
            let mut to_idx = rng.gen_range(0..account_ids.len());
            while to_idx == from_idx {
                to_idx = rng.gen_range(0..account_ids.len());
            }
            Some(account_ids[to_idx])
        } else {
            None
        };

        let amount = rng.gen_range(50.0..8000.0);

        let transaction_type = if is_transfer {
            "transfer"
        } else if rng.gen_bool(0.6) {
            "deposit"
        } else {
            "withdrawal"
        };

        let description = match transaction_type {
            "deposit" => "Depósito recibido",
            "withdrawal" => "Pago / retiro",
            "transfer" => "Transferencia interna",
            _ => "Operación",
        };

        sqlx::query(
            r#"
            INSERT INTO transactions
            (type, account_id, related_account_id, amount, description, occurred_at)
            VALUES (?, ?, ?, ?, ?, datetime('now', '-' || (abs(random()) % 180) || ' days'))
            "#,
        )
        .bind(transaction_type)
        .bind(account_id)
        .bind(related_account_id)
        .bind(amount)
        .bind(description)
        .execute(pool)
        .await
        .expect("failed to insert transaction");
    }

    println!("🎉 ¡Seed finalizado correctamente!");
}
