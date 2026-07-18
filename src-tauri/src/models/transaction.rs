use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub r#type: String,
    pub account_id: i64,
    pub related_account_id: Option<i64>,
    pub amount: f64,
    pub comment: Option<String>,
    pub occurred_at: String,
    pub created_at: String,
    pub updated_at: String,
}
