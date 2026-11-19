use crate::error::AppResult;
use crate::state::AppState;
use chrono::{Duration, Local};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedDummyOptions {
    /// Número de cuentas a generar (por defecto: 200)
    pub accounts: Option<i64>,
    /// Número de transacciones a generar (por defecto: 5000)
    pub transactions: Option<i64>,
    /// Si es true, elimina los datos existentes antes de sembrar (por defecto: true)
    pub clear_existing: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedDummyResult {
    pub accounts_created: i64,
    pub concepts_created: i64,
    pub transactions_created: i64,
    pub transaction_concepts_created: i64,
    pub cleared_existing: bool,
}

fn local_timestamp() -> String {
    Local::now()
        .naive_local()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

fn random_datetime_string(rng: &mut impl Rng, days_back: i64) -> String {
    let days = rng.gen_range(0..=days_back);
    let secs = rng.gen_range(0..86_400);
    let dt = Local::now().naive_local() - Duration::days(days) - Duration::seconds(secs);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn pick_str<'a>(rng: &mut impl Rng, items: &'a [&'a str]) -> &'a str {
    items[rng.gen_range(0..items.len())]
}

pub async fn seed_dummy_data_impl(
    db: &SqlitePool,
    options: SeedDummyOptions,
) -> AppResult<SeedDummyResult> {
    let accounts_n = options.accounts.unwrap_or(200).clamp(1, 50_000);
    let tx_n = options.transactions.unwrap_or(5_000).clamp(0, 500_000);
    let clear_existing = options.clear_existing.unwrap_or(true);

    let mut tx = db.begin().await?;

    if clear_existing {
        // Orden seguro respetando FKs (transaction_concepts y concepts se limpian por cascada,
        // pero lo hacemos explícito para robustez).
        sqlx::query("DELETE FROM transaction_concepts")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM transactions")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM concepts")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM accounts")
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "DELETE FROM sqlite_sequence WHERE name IN ('accounts', 'transactions', 'concepts')",
        )
        .execute(&mut *tx)
        .await?;
    }

    let mut rng = StdRng::from_entropy();
    let now = local_timestamp();

    let currencies = ["USD", "EUR", "MXN", "ARS", "COP", "CLP"];
    let colors = [
        "#3b82f6", "#10b981", "#f59e0b", "#ef4444", "#8b5cf6", "#ec4899", "#14b8a6",
        "#f97316", "#84cc16", "#06b6d4", "#6366f1", "#a855f7",
    ];
    let base_names = [
        "Principal",
        "Ahorros",
        "Operativa",
        "Inversiones",
        "Tesorería",
        "Proyectos",
        "Nómina",
        "Caja",
        "Reservas",
        "Servicios",
        "Ingresos",
        "Gastos",
    ];
    let destinations = [
        "Operaciones generales",
        "Pagos a proveedores",
        "Cobros de clientes",
        "Fondo de emergencia",
        "Inversiones a corto plazo",
        "Gestión administrativa",
        "Caja chica",
        "Tesorería central",
    ];
    let comments_account = [
        "Cuenta de uso diario",
        "Cuenta para movimientos operativos",
        "Cuenta de respaldo",
        "Cuenta de gestión interna",
        "Cuenta para proyectos especiales",
        "Cuenta administrativa",
    ];

    let mut account_ids: Vec<i64> = Vec::with_capacity(accounts_n as usize);

    for i in 1..=accounts_n {
        let number = format!("ACC-{:06}", i);
        let base = pick_str(&mut rng, &base_names);
        let name = format!("{} {:03}", base, i);
        let destination = if rng.gen_bool(0.7) {
            Some(pick_str(&mut rng, &destinations).to_string())
        } else {
            None
        };
        let comment = if rng.gen_bool(0.8) {
            format!("{} - {}", pick_str(&mut rng, &comments_account), i)
        } else {
            format!("Cuenta {}", i)
        };
        let currency = pick_str(&mut rng, &currencies).to_string();
        let color = if rng.gen_bool(0.9) {
            Some(pick_str(&mut rng, &colors).to_string())
        } else {
            None
        };

        let (id,): (i64,) = sqlx::query_as(
            "INSERT INTO accounts (number, name, destination, comment, currency, color, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             RETURNING id",
        )
        .bind(&number)
        .bind(&name)
        .bind(destination)
        .bind(&comment)
        .bind(&currency)
        .bind(color)
        .bind(&now)
        .bind(&now)
        .fetch_one(&mut *tx)
        .await?;

        account_ids.push(id);
    }

    // Conceptos por cuenta y tipo
    let deposit_concepts = [
        "Venta",
        "Cobro a cliente",
        "Ingreso por servicio",
        "Aporte de capital",
        "Devolución",
        "Intereses",
        "Otros ingresos",
    ];
    let withdrawal_concepts = [
        "Pago a proveedor",
        "Nómina",
        "Servicios básicos",
        "Impuestos",
        "Mantenimiento",
        "Compra de insumos",
        "Otros gastos",
    ];
    let transfer_concepts = [
        "Transferencia interna",
        "Reasignación de fondos",
        "Préstamo entre cuentas",
        "Ajuste de tesorería",
        "Cobertura operativa",
    ];

    #[derive(Clone)]
    struct ConceptBuckets {
        deposit: Vec<i64>,
        withdrawal: Vec<i64>,
        transfer: Vec<i64>,
    }

    let mut concept_map: std::collections::HashMap<i64, ConceptBuckets> =
        std::collections::HashMap::with_capacity(account_ids.len());
    let mut concepts_created: i64 = 0;

    for &account_id in &account_ids {
        let mut buckets = ConceptBuckets {
            deposit: Vec::new(),
            withdrawal: Vec::new(),
            transfer: Vec::new(),
        };

        for (ctype, names) in [
            ("deposit", &deposit_concepts[..]),
            ("withdrawal", &withdrawal_concepts[..]),
            ("transfer", &transfer_concepts[..]),
        ] {
            // 2 a todos los conceptos disponibles por tipo, para variedad
            let count = rng.gen_range(2..=names.len());
            let mut picked: Vec<&str> = names.to_vec();
            picked.shuffle(&mut rng);
            picked.truncate(count);

            for name in picked {
                let (cid,): (i64,) = sqlx::query_as(
                    "INSERT INTO concepts (account_id, type, name, is_active, created_at, updated_at)
                     VALUES (?, ?, ?, 1, ?, ?)
                     RETURNING id",
                )
                .bind(account_id)
                .bind(ctype)
                .bind(name)
                .bind(&now)
                .bind(&now)
                .fetch_one(&mut *tx)
                .await?;

                match ctype {
                    "deposit" => buckets.deposit.push(cid),
                    "withdrawal" => buckets.withdrawal.push(cid),
                    "transfer" => buckets.transfer.push(cid),
                    _ => {}
                }
                concepts_created += 1;
            }
        }

        concept_map.insert(account_id, buckets);
    }

    let tx_comments = [
        "Pago de servicios",
        "Cobro de factura",
        "Compra de materiales",
        "Ajuste contable",
        "Movimiento programado",
        "Ingreso extraordinario",
        "Gasto operativo",
        "Transferencia entre áreas",
        "Abono de cliente",
        "Pago parcial",
        "Liquidación",
        "Reposición de caja",
    ];

    let mut transaction_concepts_created: i64 = 0;
    let mut transactions_created: i64 = 0;

    for _ in 0..tx_n {
        if account_ids.is_empty() {
            break;
        }

        let account_id = account_ids[rng.gen_range(0..account_ids.len())];
        let roll: f64 = rng.gen();
        let tx_type = if roll < 0.35 {
            "deposit"
        } else if roll < 0.65 {
            "withdrawal"
        } else {
            "transfer"
        };

        let related_account_id = if tx_type == "transfer" && account_ids.len() > 1 {
            let mut related = account_ids[rng.gen_range(0..account_ids.len())];
            let mut guard = 0;
            while related == account_id && guard < 10 {
                related = account_ids[rng.gen_range(0..account_ids.len())];
                guard += 1;
            }
            if related == account_id {
                None
            } else {
                Some(related)
            }
        } else {
            None
        };

        // Si no se pudo elegir cuenta relacionada válida, degradar a depósito/retiro
        let tx_type = if tx_type == "transfer" && related_account_id.is_none() {
            if rng.gen_bool(0.5) { "deposit" } else { "withdrawal" }
        } else {
            tx_type
        };

        let amount = (rng.gen_range(10.0_f64..20_000.0_f64) * 100.0).round() / 100.0;
        let comment = if rng.gen_bool(0.75) {
            Some(pick_str(&mut rng, &tx_comments).to_string())
        } else {
            None
        };
        let occurred_at = random_datetime_string(&mut rng, 730);

        let (tx_id,): (i64,) = sqlx::query_as(
            "INSERT INTO transactions (type, account_id, related_account_id, amount, comment, occurred_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             RETURNING id",
        )
        .bind(tx_type)
        .bind(account_id)
        .bind(related_account_id)
        .bind(amount)
        .bind(comment)
        .bind(&occurred_at)
        .bind(&now)
        .bind(&now)
        .fetch_one(&mut *tx)
        .await?;

        transactions_created += 1;

        // Asociar concepto en la mayoría de los casos
        if rng.gen_bool(0.85) {
            if let Some(buckets) = concept_map.get(&account_id) {
                let concept_ids = match tx_type {
                    "deposit" => &buckets.deposit,
                    "withdrawal" => &buckets.withdrawal,
                    "transfer" => &buckets.transfer,
                    _ => &buckets.deposit,
                };
                if !concept_ids.is_empty() {
                    let concept_id = concept_ids[rng.gen_range(0..concept_ids.len())];
                    sqlx::query(
                        "INSERT INTO transaction_concepts (transaction_id, concept_id) VALUES (?, ?)",
                    )
                    .bind(tx_id)
                    .bind(concept_id)
                    .execute(&mut *tx)
                    .await?;
                    transaction_concepts_created += 1;
                }
            }
        }
    }

    tx.commit().await?;

    Ok(SeedDummyResult {
        accounts_created: account_ids.len() as i64,
        concepts_created,
        transactions_created,
        transaction_concepts_created,
        cleared_existing: clear_existing,
    })
}

#[tauri::command]
pub async fn seed_dummy_data(
    state: State<'_, AppState>,
    options: Option<SeedDummyOptions>,
) -> AppResult<SeedDummyResult> {
    seed_dummy_data_impl(&state.db, options.unwrap_or(SeedDummyOptions {
        accounts: None,
        transactions: None,
        clear_existing: None,
    }))
    .await
}
