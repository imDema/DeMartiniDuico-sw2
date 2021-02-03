
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgDone;
use sqlx::{FromRow, PgPool, query_as, query};
use chrono::prelude::*;

use futures::StreamExt;

use crate::utils::encoding::encode_serial;

#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    pub id: i32,
    pub customer_id: i32,
    pub shop_id: i32,
    pub shop_name: String,
    pub creation: NaiveDateTime,
    pub expiration: NaiveDateTime,
    pub est_minutes: i32,
    pub valid: bool,
    pub active: bool,
    pub department_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TicketResponse {
    pub uid: String,
    pub shop_id: String,
    pub shop_name: String,
    pub department_ids: Vec<String>,
    pub creation: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub valid: bool,
    pub active: bool,
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
            shop_name: t.shop_name,
            department_ids: dids,
            creation: Utc.from_utc_datetime(&t.creation),
            expiration: Utc.from_utc_datetime(&t.expiration),
            valid: t.valid,
            active: t.active,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum EnterResult {
    Entered,
    Full(i32),
    NotFirst(i64),
    Expired,
    Invalid,
}

pub enum NewTicketResult<'a> {
    Created(PersistentTicket<'a>),
    AlreadyExists,
    Closed,
}
impl<'a> NewTicketResult<'a> {
    pub fn unwrap(self) -> PersistentTicket<'a> {
        match self {
            NewTicketResult::Created(t) => t,
            NewTicketResult::AlreadyExists => panic!("Unwrap called on AlreadyExists result"),
            NewTicketResult::Closed => panic!("Unwrap called on Closed result"),
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
        let ticket = query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, shop.name as shop_name, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active
            FROM ticket, ticket_department, department, shop
            WHERE ticket_department.ticket_id = ticket.id AND
                ticket.shop_id = shop.id AND
                ticket_department.department_id = department.id AND
                ticket.id = $1
            GROUP BY ticket.id, customer_id, ticket.shop_id, shop.name, creation, expiration, valid, active",
            id)
            .fetch_optional(conn)
            .await?
            .map(move |row| Self{conn, inner:row.into()});

        Ok(ticket)
    }

    pub async fn get_for_customer(conn: &'a PgPool, customer_id: i32) -> sqlx::Result<Vec<Ticket>> {
        query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, shop.name as shop_name, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active
            FROM ticket, ticket_department, department, shop
            WHERE ticket_department.ticket_id = ticket.id AND
                ticket.shop_id = shop.id AND
                ticket_department.department_id = department.id AND
                ticket.customer_id = $1
            GROUP BY ticket.id, customer_id, ticket.shop_id, shop.name, creation, expiration, valid, active
            ORDER BY creation",
            customer_id)
            .fetch(conn)
            .fold(Ok(Vec::new()), |acc: sqlx::Result<Vec<Ticket>>, x| async {
                let mut acc = acc?;
                acc.push(x?.into());
                Ok(acc)
            }).await
    }

    pub async fn new(conn: &'a PgPool, customer_id: i32, shop_id: i32, department_ids: Vec<i32>, est_minutes: i32) -> sqlx::Result<NewTicketResult<'a>> {
        let mut tx = conn.begin().await?;

        let already_have = query!(r"SELECT id FROM ticket
            WHERE
                customer_id = $1 AND shop_id = $2 AND
                exit IS NULL AND expiration > CURRENT_TIMESTAMP",
                customer_id, shop_id)
            .fetch_optional(&mut tx).await?;

        if let Some(_) = already_have {
            return Ok(NewTicketResult::AlreadyExists);
        }

        let row = query!(r"INSERT INTO ticket (customer_id, shop_id, creation, expiration, est_minutes, valid, active) VALUES
            ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + interval '6 hour', $3, TRUE, TRUE)
            RETURNING id",
            customer_id, shop_id, est_minutes)
            .fetch_one(&mut tx).await?;

        for did in department_ids {
            query!(r"INSERT INTO ticket_department (ticket_id, department_id)
                VALUES ($1, $2)",
                row.id, did)
                .execute(&mut tx).await?;
        }

        let ticket_row = query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, shop.name as shop_name, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active
            FROM ticket, ticket_department, department, shop
            WHERE
                ticket_department.ticket_id = ticket.id AND
                ticket.shop_id = shop.id AND
                ticket_department.department_id = department.id AND
                ticket.id = $1
            GROUP BY ticket.id, customer_id, ticket.shop_id, shop.name, creation, expiration, valid, active",
            row.id)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(NewTicketResult::Created(Self{conn, inner:ticket_row.into()}))
    }

    pub async fn cancel(self) -> sqlx::Result<PgDone> {
        query!("DELETE FROM ticket WHERE id = $1", self.inner.id)
            .execute(self.conn)
            .await
    }

    pub async fn queue(conn: &PgPool, shop_id: i32, valid: bool, active: bool) -> sqlx::Result<Vec<Ticket>> {
        query_as!(TicketRow, r"SELECT ticket.id AS id, customer_id, ticket.shop_id AS shop_id, shop.name as shop_name, array_agg(department.id) AS department_ids, creation, expiration, entry, exit, est_minutes, valid, active
                FROM ticket, ticket_department, department, shop
                WHERE
                    ticket.shop_id = $1 AND
                    ticket.shop_id = shop.id AND
                    ticket_department.ticket_id = ticket.id AND
                    ticket_department.department_id = department.id AND
                    valid = $2 AND active = $3
                GROUP BY ticket.id, customer_id, ticket.shop_id, shop.name, creation, expiration, valid, active
                ORDER BY creation",
                shop_id, valid, active)
            .fetch(conn)
            .fold(Ok(Vec::new()), |acc: sqlx::Result<Vec<Ticket>>, x| async {
                let mut acc = acc?;
                acc.push(x?.into());
                Ok(acc)
            }).await
    }

    pub async fn enter(&self) -> sqlx::Result<EnterResult> {
        let mut tx = self.conn.begin().await?;

        let state = query!(r"SELECT entry IS NOT NULL as entered, exit IS NOT NULL as exited FROM ticket
            WHERE id = $1", self.inner.id)
            .fetch_one(&mut tx)
            .await?;

        // TODO: Clearer distinction of validity and expiration
        if state.exited.unwrap() {
            return Ok(EnterResult::Expired);
        }
        if state.entered.unwrap() {
            return Ok(EnterResult::Invalid);
        }

        let position = query!(r"SELECT count(*) as count FROM ticket
            WHERE
                shop_id = $1 AND
                valid = TRUE AND active = TRUE AND
                entry IS NULL AND exit IS NULL AND
                id <> $2 AND creation < $3", self.inner.shop_id, self.inner.id, self.inner.creation)
            .fetch_one(&mut tx)
            .await?
            .count.unwrap();

        if position > 0 {
            return Ok(EnterResult::NotFirst(position));
        }

        let rows = query!(r"SELECT department.id as id, (count(ticket.id) >= department.capacity) as full FROM ticket, ticket_department, department
                        WHERE
                            ticket_department.ticket_id = ticket.id AND
                            ticket_department.department_id = department.id AND
                            ticket.shop_id = $1 AND
                            department.shop_id = $1 AND
                            ticket.entry IS NOT NULL AND
                            ticket.exit IS NULL
                        GROUP BY
                            department.id, department.capacity", self.inner.shop_id)
            .fetch_all(&mut tx)
            .await?;

        let full = rows.iter().find(|&row| row.full.unwrap_or(true));
        if let Some(dep) = full {
            return Ok(EnterResult::Full(dep.id));
        }

        query!(r"UPDATE ticket
            SET
                entry = CURRENT_TIMESTAMP
            WHERE id = $1", self.inner.id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(EnterResult::Entered)
    }

    pub async fn exit(&self) -> sqlx::Result<bool> {
        let mut tx = self.conn.begin().await?;

        let state = query!(r"SELECT entry IS NOT NULL as entered, exit IS NOT NULL as exited FROM ticket
            WHERE id = $1", self.inner.id)
            .fetch_one(&mut tx)
            .await?;

        if state.entered.unwrap() && !state.exited.unwrap() {
            query!(r"UPDATE ticket
                SET
                    exit = CURRENT_TIMESTAMP
                WHERE id = $1", self.inner.id)
                .execute(&mut tx)
                .await?;

            tx.commit().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn inner(&self) -> &Ticket {&self.inner}
    pub fn into_inner(self) -> Ticket {self.inner}
}

#[derive(FromRow)]
pub(super) struct TicketRow {
    pub id: i32,
    pub customer_id: i32,
    pub shop_id: i32,
    pub shop_name: String,
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
            shop_name: row.shop_name,
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
    use crate::utils::tests::*;
    use crate::with_test_shop;

    
    #[actix_rt::test]
    async fn new_ticket_test() -> Result<(), Box<dyn Error>> {
        let conn = db().await;

        with_test_shop!(&conn, shopid [d1, d2] {
            let customer_id = test_customer(&conn).await?;

            let inserted = PersistentTicket::new(&conn, customer_id, shopid, vec![d1, d2], 25)
                .await?.unwrap().into_inner();
    
            let loaded = PersistentTicket::get(&conn, inserted.id).await?.map(PersistentTicket::into_inner);
            let loaded = loaded.unwrap();
            assert_eq!(&inserted, &loaded);
        });
        Ok(())
    }

    #[actix_rt::test]
    async fn duplicate_ticket_test() -> Result<(), Box<dyn Error>> {
        let conn = db().await;

        with_test_shop!(&conn, s0 [d0, d1], s1 [d2] {
            let customer_id = test_customer(&conn).await?;

            let _ = PersistentTicket::new(&conn, customer_id, s0, vec![d0], 25).await?.unwrap();

            match PersistentTicket::new(&conn, customer_id, s0, vec![d1], 25).await? {
                NewTicketResult::AlreadyExists => {},
                _ => panic!("Expected AlreadyExists"),
            }
            let _ = PersistentTicket::new(&conn, customer_id, s1, vec![d2], 25).await?.unwrap();
    
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
                .await?.unwrap().into_inner();

            let t2 = PersistentTicket::new(&conn, id_c2, shopid, vec![d1,d2,d3], 25)
                .await?.unwrap().into_inner();

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

    #[actix_rt::test]
    async fn entry_exit_test() -> Result<(), Box<dyn Error>>{
        let conn = db().await;

        let id_c1 = test_customer(&conn).await?;
        let id_c2 = test_customer(&conn).await?;
        let id_c3 = test_customer(&conn).await?;

        with_test_shop!(&conn, shopid [d0, d1] {
            let d_small = test_department(&conn, shopid, 2).await?;

            let t1 = PersistentTicket::new(&conn, id_c1, shopid, vec![d_small], 25).await?.unwrap();
            let t2 = PersistentTicket::new(&conn, id_c2, shopid, vec![d_small, d0], 25).await?.unwrap();
            let t3 = PersistentTicket::new(&conn, id_c3, shopid, vec![d_small, d1], 25).await?.unwrap();

            assert_eq!(t1.exit().await.unwrap(), false);

            assert_eq!(t2.enter().await.unwrap(), EnterResult::NotFirst(1));
            assert_eq!(t3.enter().await.unwrap(), EnterResult::NotFirst(2));

            assert_eq!(t1.enter().await.unwrap(), EnterResult::Entered);
            assert_eq!(t3.enter().await.unwrap(), EnterResult::NotFirst(1));

            assert_eq!(t2.enter().await.unwrap(), EnterResult::Entered);
            assert_eq!(t3.enter().await.unwrap(), EnterResult::Full(d_small));

            assert_eq!(t2.exit().await.unwrap(), true);
            assert_eq!(t3.enter().await.unwrap(), EnterResult::Entered);

            assert_eq!(t1.enter().await.unwrap(), EnterResult::Invalid);

            assert_eq!(t1.exit().await.unwrap(), true);
            assert_eq!(t3.exit().await.unwrap(), true);
            assert_eq!(t3.exit().await.unwrap(), false);

            assert_eq!(t1.enter().await.unwrap(), EnterResult::Expired);
            assert_eq!(t2.enter().await.unwrap(), EnterResult::Expired);
            assert_eq!(t3.enter().await.unwrap(), EnterResult::Expired);

        });
        Ok(())
    }
}