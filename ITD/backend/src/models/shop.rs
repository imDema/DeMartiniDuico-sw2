
use sqlx::postgres::PgRow;
use sqlx::postgres::types::PgTimeTz;
use sqlx::{FromRow, PgPool, Row, query};
use sqlx::query_as;

use futures::StreamExt;

use super::ticket::Ticket;
#[allow(dead_code)]
#[derive(FromRow)]
pub struct Shop {
    id: i32,
    name: String,
    description: String,
    image: Option<String>,
    location: String,
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct Department {
    id: i32,
    shop_id: i32,
    description: String,
    capacity: i32
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct Schedule {
    shop_id: i32,
    dow: i16,
    open: PgTimeTz,
    close: PgTimeTz,
}

pub struct PersistentShop<'a> {
    conn: &'a PgPool,
    inner: Shop,
}

impl<'a> PersistentShop<'a> {
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentShop<'a>>> {
        let acc = query_as!(Shop,
            r"SELECT id, name, description, image, location FROM shop
            WHERE id = $1",
            id
        ).fetch_optional(conn)
        .await?;

        Ok(acc.map(|acc| Self{inner: acc, conn}))
    }

    pub async fn schedule(&self) -> sqlx::Result<Vec<Schedule>> {
        Ok(query_as!(Schedule,
            r"SELECT shop_id, dow, open, close FROM schedule
            WHERE shop_id = $1
            ORDER BY dow, open",
            self.inner.id
        ).fetch_all(self.conn)
        .await?)
    }

    pub async fn departments(&self) -> sqlx::Result<Vec<Department>> {
        Ok(query_as!(Department,
            r"SELECT id, shop_id, description, capacity FROM department
            WHERE shop_id = $1",
            self.inner.id
        ).fetch_all(self.conn)
        .await?)
    }

    fn parse_ticket_join(row: &PgRow) -> sqlx::Result<Ticket> {
        let tick = Ticket{
            id: row.try_get(0)?,
            shop_id: row.try_get(1)?,
            department_ids: vec![row.try_get(2)?],
            creation: row.try_get("creation")?,
            expiration: row.try_get("expiration")?,
            valid: row.try_get("valid")?,
            active: row.try_get("active")?,
        };
        Ok(tick)
    }

    pub async fn active_queue(&self) -> sqlx::Result<Vec<Ticket>> { // TODO: refactor
        let tickets = query(r"SELECT ticket.id, ticket.shop_id, department.id, creation, expiration, valid, active FROM ticket, ticket_department, department
                WHERE
                    ticket.shop_id = $1 AND
                    ticket_department.ticket_id = ticket.id AND
                    ticket_department.department_id = department.id AND
                    valid AND active
                ORDER BY creation")
            .bind(self.inner.id)
            .fetch(self.conn)
            .fold(Ok(Vec::new()), |acc: sqlx::Result<Vec<Ticket>>, row| async {
                acc.and_then(|mut acc| {
                    row.and_then(|row| {
                        if let Some(last) = acc.last_mut() {
                            let id: i32 = row.try_get(0)?;
                            if id == last.id {
                                last.department_ids.push(row.try_get(2)?);
                                return Ok(acc);
                            }
                        }
                        let tick = Self::parse_ticket_join(&row)?;
                        acc.push(tick);
                        Ok(acc)
                    }).map_err(|err| {log::error!("{}", &err); err})
                })
            }).await;

        tickets
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn queue_test() {
        dotenv::dotenv().ok();
        let conn_url = std::env::var("DATABASE_URL").unwrap();
        let db = crate::setup_db(&conn_url).await;
        
        let shop = PersistentShop::get(&db, 1234111).await.unwrap().unwrap();

        for t in shop.active_queue().await.unwrap() {
            println!("{:?}", t);
        }

        let shop = PersistentShop::get(&db, 1234222).await.unwrap().unwrap();

        for t in shop.active_queue().await.unwrap() {
            println!("{:?}", t);
        }
    }
}