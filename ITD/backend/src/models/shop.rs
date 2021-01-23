
use sqlx::postgres::types::PgTimeTz;
use sqlx::{FromRow, PgPool};
use sqlx::{query, query_as};

use super::ticket::{Ticket, TicketRow, TicketJoinRow, fold_ticketjoin_stream};

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

    pub async fn new_ticket(&self, customer_id: i32, department_ids: Vec<i32>) -> sqlx::Result<Option<Ticket>> {
        let mut tx = self.conn.begin().await?;

        let row = query_as!(TicketRow, r"INSERT INTO ticket (customer_id, shop_id, creation, expiration, valid, active) VALUES
            ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, TRUE, TRUE)
            RETURNING id, customer_id, shop_id, creation, expiration, valid, active",
            customer_id, self.inner.id)
            .fetch_one(&mut tx).await?;

        let mut ticket = Ticket::from(row);
        for did in department_ids {
            query!(r"INSERT INTO ticket_department (ticket_id, department_id)
                VALUES ($1, $2)",
                ticket.id, did)
                .execute(&mut tx).await?;
            ticket.department_ids.push(did);
        }

        tx.commit().await?;
        Ok(Some(ticket))
    }

    pub async fn active_queue(&self) -> sqlx::Result<Vec<Ticket>> {
        let ticket_stream = query_as!(TicketJoinRow, r"SELECT ticket.id AS ticket_id, customer_id, ticket.shop_id AS shop_id, department.id AS department_id, creation, expiration, valid, active FROM ticket, ticket_department, department
                WHERE
                    ticket.shop_id = $1 AND
                    ticket_department.ticket_id = ticket.id AND
                    ticket_department.department_id = department.id AND
                    valid AND active
                ORDER BY creation",
                self.inner.id)
            .fetch(self.conn);

        fold_ticketjoin_stream(ticket_stream).await
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::time::Duration;

    use super::*;
    use sqlx::postgres::PgDone;
    use sqlx::{query_as, query};

    async fn init_test_shops(conn: &PgPool) -> sqlx::Result<Vec<i32>> {
        query!(r"INSERT INTO shop (id, name, description, location) VALUES
                (1234111, 'Unes Milano', 'Unes via unes numero unes','49.1234N,12.3456E'),
                (1234222, 'Lidl Torino', 'Lidl via lidl numero lidl','123.1234N,45.3456E'),
                (1234333, 'Fruttivendolo da Attilio', 'Frutta e verdura','2.1234S,23.3456W');")
            .execute(conn)
            .await?;
        Ok(vec![1234111, 1234222, 1234333])
    }

    async fn drop_test_shops(conn: &PgPool) -> sqlx::Result<PgDone> {
        query!(r"DELETE FROM shop WHERE
                    id = 1234111 OR id = 1234222 OR id = 1234333;")
            .execute(conn)
            .await
    }

    #[actix_rt::test]
    async fn queue_test() -> Result<(), Box<dyn Error>>{
        dotenv::dotenv().ok();
        let conn_url = std::env::var("DATABASE_URL").unwrap();
        let conn = crate::setup_db(&conn_url).await;
        // let ids = init_test_shops(&conn).await?; TODO:

        let shop = PersistentShop::get(&conn, 1234111).await?.unwrap();

        // TODO: parametrize

        let t1 = shop.new_ticket(1111222, vec![4444111, 4444222])
            .await?.unwrap();

        std::thread::sleep(Duration::from_millis(10));
        let t2 = shop.new_ticket(1111333, vec![4444222])
            .await?.unwrap();

        let queue = shop.active_queue().await?;

        for t in queue.iter() {
            println!("{:?}", t);
        }
        // assert_eq!(2, queue.len()); TODO:
        assert!(queue.contains(&t1));
        assert!(queue.contains(&t2));

        query!("DELETE FROM ticket WHERE id = $1 OR id = $2", t1.id, t2.id)
            .execute(&conn).await?;

        // drop_test_shops(&conn);
        Ok(())
    }
}