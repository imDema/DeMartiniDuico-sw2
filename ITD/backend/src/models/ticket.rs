use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Ticket {
    pub id: i32,
    pub shop_id: i32,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub valid: bool,
    pub active: bool,
    pub department_ids: Vec<i32>,
}

#[derive(FromRow)]
pub(super) struct TicketRow {
    pub id: i32,
    pub shop_id: i32,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub valid: bool,
    pub active: bool,
} 

#[derive(FromRow, Serialize, Deserialize)]
pub struct TicketDepartment {
    pub ticket_id: i32,
    pub department_id: i32,
}

