
use serde::Serialize;
use sqlx::{FromRow, PgPool, query_as, query};
use chrono::prelude::*;

use futures::StreamExt;

use crate::utils::encoding::encode_serial;

#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    pub id: i32,
    pub customer_id: i32,
    pub shop_id: i32,
    pub creation: NaiveDateTime,
    pub expiration: NaiveDateTime,
    pub est_minutes: i32,
    pub valid: bool,
    pub active: bool,
    pub department_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct TicketResponse {
    uid: String,
    shop_id: String,
    department_ids: Vec<String>,
    creation: NaiveDateTime,
    expiration: NaiveDateTime,
    valid: bool,
    active: bool,
}

impl From<Ticket> for TicketResponse {
    fn from(t: Ticket) -> Self {
        let dids = t.department_ids
            .into_iter()
            .map(encode_serial)
            .collect();
        Self {
            uid: encode_serial(t.id),
            shop_id: encode_serial(t.shop_id),
            department_ids: dids,
            creation: t.creation,
            expiration: t.expiration,
            valid: t.valid,
            active: t.active,
        }
    }
}

#[allow(dead_code)]
pub struct PersistentTicket<'a> {
    conn: &'a PgPool,
    inner: Ticket,
}

impl<'a> PersistentTicket<'a> {
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentTicket<'a>>> {
        let ticket = query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active FROM ticket, ticket_department, department
            WHERE ticket_department.ticket_id = ticket.id AND
                ticket_department.department_id = department.id AND
                ticket.id = $1
            GROUP BY ticket.id, customer_id, ticket.shop_id, creation, expiration, valid, active",
            id)
            .fetch_optional(conn)
            .await?
            .map(move |row| Self{conn, inner:row.into()});

        Ok(ticket)
    }

    pub async fn get_for_customer(conn: &'a PgPool, customer_id: i32) -> sqlx::Result<Vec<Ticket>> {
        query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active FROM ticket, ticket_department, department
            WHERE ticket_department.ticket_id = ticket.id AND
                ticket_department.department_id = department.id AND
                ticket.customer_id = $1
            GROUP BY ticket.id, customer_id, ticket.shop_id, creation, expiration, valid, active
            ORDER BY creation",
            customer_id)
            .fetch(conn)
            .fold(Ok(Vec::new()), |acc: sqlx::Result<Vec<Ticket>>, x| async {
                let mut acc = acc?;
                acc.push(x?.into());
                Ok(acc)
            }).await
    }

    pub async fn new(conn: &'a PgPool, customer_id: i32, shop_id: i32, department_ids: Vec<i32>, est_minutes: i32) -> sqlx::Result<PersistentTicket<'a>> {
        let mut tx = conn.begin().await?;

        let row = query!(r"INSERT INTO ticket (customer_id, shop_id, creation, expiration, est_minutes, valid, active) VALUES
            ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $3, TRUE, TRUE)
            RETURNING id",
            customer_id, shop_id, est_minutes)
            .fetch_one(&mut tx).await?;

        for did in department_ids {
            query!(r"INSERT INTO ticket_department (ticket_id, department_id)
                VALUES ($1, $2)",
                row.id, did)
                .execute(&mut tx).await?;
        }

        tx.commit().await?;
        let ticket = PersistentTicket::get(conn, row.id).await?.unwrap(); // TODO: check unwrap

        Ok(ticket)
    }

    pub async fn queue(conn: &PgPool, shop_id: i32, valid: bool, active: bool) -> sqlx::Result<Vec<Ticket>> {
        query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active FROM ticket, ticket_department, department
                WHERE
                    ticket.shop_id = $1 AND
                    ticket_department.ticket_id = ticket.id AND
                    ticket_department.department_id = department.id AND
                    valid = $2 AND active = $3
                GROUP BY ticket.id, customer_id, ticket.shop_id, creation, expiration, valid, active
                ORDER BY creation",
                shop_id, valid, active)
            .fetch(conn)
            .fold(Ok(Vec::new()), |acc: sqlx::Result<Vec<Ticket>>, x| async {
                let mut acc = acc?;
                acc.push(x?.into());
                Ok(acc)
            }).await
    }

    pub fn into_inner(self) -> Ticket {self.inner}
}

#[derive(FromRow)]
pub(super) struct TicketRow {
    pub id: i32,
    pub customer_id: i32,
    pub shop_id: i32,
    pub creation: NaiveDateTime,
    pub expiration: NaiveDateTime,
    pub entry: Option<NaiveDateTime>,
    pub exit: Option<NaiveDateTime>,
    pub est_minutes: i32,
    pub valid: bool,
    pub active: bool,
    pub department_ids: Option<Vec<i32>>,
} 

impl From<TicketRow> for Ticket {
    fn from(row: TicketRow) -> Self {
        Ticket {
            id: row.id,
            customer_id: row.customer_id,
            shop_id: row.shop_id,
            creation: row.creation.into(),
            expiration: row.expiration,
            est_minutes: row.est_minutes,
            valid: row.valid,
            active: row.active,
            department_ids: row.department_ids.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::time::Duration;
    use crate::utils::tests::{db, test_customer};
    use crate::with_test_shop;

    #[actix_rt::test]
    async fn new_ticket_test() -> Result<(), Box<dyn Error>> {
        let conn = db().await;

        with_test_shop!(&conn, shopid [d1, d2] {
            let customer_id = test_customer(&conn).await?;

            let inserted = PersistentTicket::new(&conn, customer_id, shopid, vec![d1, d2], 25)
                .await?.into_inner();
    
            let loaded = PersistentTicket::get(&conn, inserted.id).await?.map(PersistentTicket::into_inner);
            let loaded = loaded.unwrap();
            assert_eq!(&inserted, &loaded);
        });
        Ok(())
    }
    
    #[actix_rt::test]
    async fn queue_test() -> Result<(), Box<dyn Error>>{
        let conn = db().await;

        let id_c1 = test_customer(&conn).await?;
        let id_c2 = test_customer(&conn).await?;

        with_test_shop!(&conn, shopid [d0, d1, d2, d3] {
            let t1 = PersistentTicket::new(&conn, id_c1, shopid, vec![d0, d3], 25)
                .await?.into_inner();

            std::thread::sleep(Duration::from_millis(10));
            let t2 = PersistentTicket::new(&conn, id_c2, shopid, vec![d1,d2,d3], 25)
                .await?.into_inner();

            let queue = PersistentTicket::queue(&conn, shopid, true, true).await?;

            for t in queue.iter() {
                println!("{:?}", t);
            }
            assert_eq!(2, queue.len());
            assert!(queue.contains(&t1));
            assert!(queue.contains(&t2));

            assert_eq!(Some(&t1), queue.first());

            query!("DELETE FROM ticket WHERE id = $1 OR id = $2", t1.id, t2.id)
                .execute(&conn).await?;

        });
        Ok(())
    }
}