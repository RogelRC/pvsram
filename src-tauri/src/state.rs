use sqlx::SqlitePool;
use std::sync::Mutex;

pub struct AppState {
    pub db: SqlitePool,
    pub authenticated: Mutex<bool>,
}
