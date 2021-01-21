use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::prelude::*;

use super::shop::Department;

pub struct Ticket {
    pub id: i32,
    pub shop_id: i32,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub valid: bool,
    pub active: bool,
    pub departments: Vec<Department>,
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

