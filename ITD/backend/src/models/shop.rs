
use sqlx::postgres::types::PgTimeTz;
use sqlx::{FromRow, PgPool};
use sqlx::query_as;

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

    pub async fn queue(&self) -> sqlx::Result<Vec<Ticket>> {
        Ok(query_as!(Ticket,
            r"SELECT id, shop_id, creation, expiration, valid FROM ticket
            WHERE shop_id = $1
            ORDER BY creation",
            self.inner.id
        ).fetch_all(self.conn)
        .await?)
    }
}