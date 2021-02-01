use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool, query, query_as};
use rand::Rng;

use super::account::Account;

type ConfirmationCode = Vec<u8>;

pub struct PersistentCustomer<'a> {
    inner: Account,
    conn: &'a PgPool,
}

impl<'a> PersistentCustomer<'a> {
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentCustomer<'a>>> {
        let acc = query_as!(Account,
            r"SELECT id, email, salt, digest FROM customer WHERE id = $1",
            id
        ).fetch_optional(conn)
        .await?;

        Ok(acc.map(|acc|Self{inner: acc, conn}))
    }

    pub async fn find(conn: &'a PgPool, email: &str) -> sqlx::Result<Option<PersistentCustomer<'a>>> {
        let acc = query_as!(Account,
                r"SELECT id, email, salt, digest FROM customer WHERE email = $1",
                email
            ).fetch_optional(conn)
            .await?;
        if let Some(acc) = acc {
            Ok(Some(Self{inner: acc, conn}))
        } else {
            Ok(None)
        }
    }

    /// Create a new customer if no other customer with the same email exists
    /// # Returns:
    /// `Ok(Some(PersistentCustomer))` if it was created
    /// `Ok(None)` if an customer with the same email already existed
    pub async fn create(conn: &'a PgPool, email: &str, password: &str) -> sqlx::Result<Option<ConfirmationCode>> {
        let mut tx = conn.begin().await?;

        let exists = query!(r"SELECT email FROM customer WHERE email = $1", &email)
            .fetch_optional(&mut tx)
            .await?;
        
        if let None = exists {
            let p = Account::hash_password(password.as_bytes());
            let mut code = vec![0u8; 32];
            rand::thread_rng().fill(&mut code[..]);
            let acc =  query_as!(TempCustomer,
                    r"INSERT INTO temp_customer(code, email, salt, digest) VALUES ($1, $2, $3, $4) RETURNING code, email, salt, digest",
                    &code, &email, &p.salt, &p.digest
                ).fetch_one(&mut tx)
                .await?;
            tx.commit().await?;
            Ok(Some(acc.code))
        } else {
            tx.rollback().await?;
            Ok(None)
        }
    }

    pub async fn finalize(conn: &'a PgPool, code: &[u8]) -> sqlx::Result<Option<Account>> {
        let mut tx = conn.begin().await?;

        let temp = query!(r"SELECT code, email, salt, digest FROM temp_customer WHERE code = $1", code)
            .fetch_optional(&mut tx)
            .await?;
        
        let result = if let Some(temp) = temp {
            let acc = query_as!(Account,
                    r"INSERT INTO customer(email, salt, digest) VALUES ($1, $2, $3) RETURNING id, email, salt, digest",
                    &temp.email, &temp.salt, &temp.digest
                ).fetch_one(&mut tx)
                .await?;

            query!(r"DELETE FROM temp_customer WHERE code = $1", &code)
                .execute(&mut tx)
                .await?;

            Ok(Some(acc))
        } else {
            Ok(None)
        };

        tx.commit().await?;
        result
    }

    pub async fn update_password(&'a mut self, password: &str) -> sqlx::Result<&'a mut PersistentCustomer<'a>> {
        let p = Account::hash_password(password.as_bytes());
        let acc =  query_as!(Account,
                r"UPDATE customer SET salt = $1, digest = $2 WHERE id = $3 RETURNING id, email, salt, digest",
                &p.salt, &p.digest, &self.inner.id()
            ).fetch_one(self.conn)
            .await?;
        self.inner = acc;
        Ok(self)
    }

    pub fn into_inner(self) -> Account {self.inner}
    pub fn inner(&self) -> &Account {&self.inner}
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TempCustomer {
    code: Vec<u8>,
    email: String,
    salt: Vec<u8>,
    digest: Vec<u8>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::{db, del_customer};

    #[actix_rt::test]
    async fn create_customer_test() -> sqlx::Result<()> {
        let conn = db().await;
        let (email, password) = ("test-email123@mail.com", "securepassword");

        let token = PersistentCustomer::create(&conn, email, password)
            .await?
            .expect("No temporary account was created");

        let account = PersistentCustomer::finalize(&conn, &token)
            .await?
            .expect("No customer was created");

        assert_eq!(email, account.email());

        let loaded = PersistentCustomer::get(&conn, account.id())
            .await?
            .expect("Could not find the recently created account!")
            .into_inner();

        assert_eq!(account, loaded);
        
        del_customer(&conn, account.id()).await?;

        Ok(())
    }
}
