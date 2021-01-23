use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::prelude::*;

use futures::{Stream, StreamExt};

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
pub(super) struct TicketJoinRow {
    pub ticket_id: i32,
    pub shop_id: i32,
    pub department_id: i32,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub valid: bool,
    pub active: bool,
}

impl From<TicketJoinRow> for Ticket {
    fn from(row: TicketJoinRow) -> Self {
        Ticket {
            id: row.ticket_id,
            shop_id: row.shop_id,
            creation: row.creation,
            expiration: row.expiration,
            valid: row.valid,
            active: row.active,
            department_ids: vec![row.department_id],
        }
    }
}

pub(super) async fn fold_ticketjoin_stream<S: Stream<Item=sqlx::Result<TicketJoinRow>>>(rows: S) -> sqlx::Result<Vec<Ticket>> {
    rows.fold(Ok(Vec::new()), |acc: sqlx::Result<Vec<Ticket>>, row| async {
        let mut acc = acc?;
        let row = row?;
        if let Some(last) = acc.last_mut() {
            if row.ticket_id == last.id {
                last.department_ids.push(row.department_id);
                return Ok(acc);
            }
        }
        acc.push(row.into());
        Ok(acc)
    }).await
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

