pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod state;

use commands::account::{
    create_account, delete_account, get_account, list_accounts, update_account,
};
use commands::transaction::{
    create_deposit, create_transfer, create_withdrawal, delete_transaction, get_account_balance,
    get_transaction, list_transactions,
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
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let pool = db::init_db(&handle).await;
                handle.manage(AppState { db: pool });
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
            list_transactions,
            get_transaction,
            delete_transaction,
            get_account_balance
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
