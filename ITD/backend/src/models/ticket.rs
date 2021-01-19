use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::prelude::*;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i32,
    pub shop_id: i32,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub valid: bool,
}