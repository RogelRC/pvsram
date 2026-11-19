pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod state;
use commands::account::{
    create_account, delete_account, get_account, list_accounts, update_account,
};
use commands::auth::{
    change_password, has_password, is_authenticated, login, logout, set_initial_password,
};
use commands::concept::{create_concept, ensure_concept, list_concepts};
use commands::database::{export_database, import_database};
use commands::seed::seed_dummy_data;
use commands::settings::{get_window_mode, set_window_mode};
use commands::transaction::{
    count_transactions_report, create_deposit, create_transfer, create_withdrawal,
    delete_transaction, get_account_balance, get_monthly_balance_report, get_movements_report,
    get_stats_by_account, get_stats_by_currency, get_transaction, get_transactions_stats,
    list_transactions, list_transactions_report, update_transaction,
};
use state::AppState;
use tauri::Manager;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = db::init_db(&handle).await;

                let mode = sqlx::query_scalar::<_, String>(
                    "SELECT window_mode FROM app_settings WHERE id = 1",
                )
                .fetch_optional(&pool)
                .await
                .ok()
                .flatten()
                .unwrap_or_else(|| "fullscreen".to_string());

                handle.manage(AppState {
                    db: pool,
                    authenticated: std::sync::Mutex::new(false),
                });

                if let Some(window) = handle.get_webview_window("main") {
                    match mode.as_str() {
                        "maximized" => {
                            let _ = window.set_fullscreen(false);
                            let _ = window.maximize();
                        }
                        "windowed" => {
                            let _ = window.set_fullscreen(false);
                            let _ = window.unmaximize();
                        }
                        _ => {
                            let _ = window.unmaximize();
                            let _ = window.set_fullscreen(true);
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            create_account,
            list_accounts,
            get_account,
            update_account,
            delete_account,
            create_deposit,
            create_withdrawal,
            create_transfer,
            list_concepts,
            create_concept,
            ensure_concept,
            list_transactions,
            get_transaction,
            delete_transaction,
            get_account_balance,
            list_transactions_report,
            count_transactions_report,
            get_transactions_stats,
            get_stats_by_account,
            get_stats_by_currency,
            get_movements_report,
            get_monthly_balance_report,
            update_transaction,
            has_password,
            set_initial_password,
            login,
            logout,
            is_authenticated,
            change_password,
            export_database,
            import_database,
            seed_dummy_data,
            get_window_mode,
            set_window_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
