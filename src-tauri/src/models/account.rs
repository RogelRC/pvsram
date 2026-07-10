use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: i64,
    pub number: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub currency: String,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
