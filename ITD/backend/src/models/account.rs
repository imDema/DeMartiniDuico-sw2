use std::pin::Pin;

use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool, Postgres, Transaction, query, query_as};
use futures::Stream;

use rand::Rng;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Customer {
    id: i32,
    email: String,
    salt: Vec<u8>,
    digest: Vec<u8>,
}

pub struct HashedPass {
    salt: Vec<u8>,
    digest: Vec<u8>,
}

impl Customer {
    fn argon_conf<'a>() -> argon2::Config<'a> { argon2::Config::default() }

    pub fn verify_authentication(&self, password: &[u8]) -> bool {
        let conf = Self::argon_conf();
        let matches = argon2::verify_raw(password, &self.salt, &self.digest, &conf);
        matches.unwrap_or_else(|err| {log::error!("Argon2 error!: `{:?}`", err); false})
    }

    pub fn hash_password(password: &[u8]) -> HashedPass {
        let config = Self::argon_conf();
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill(&mut salt[..]);
        let digest = argon2::hash_raw(password, &salt, &config).expect("Argon2 error!");
        HashedPass{salt, digest}
    }

    pub fn id(&self) -> i32 {self.id}
    pub fn email(&self) -> &str {&self.email}
    pub fn salt(&self) -> &[u8] {&self.salt[..]}
    pub fn digest(&self) -> &[u8] {&self.digest[..]}
}

type ConfirmationCode = Vec<u8>;

pub struct PersistentCustomer<'a> {
    inner: Customer,
    conn: &'a PgPool,
}

impl<'a> PersistentCustomer<'a> {
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentCustomer<'a>>> {
        let acc = query_as!(Customer,
                r"SELECT id, email, salt, digest FROM customer WHERE id = $1",
                id
            ).fetch_optional(conn)
            .await?;

        match acc {
            Some(acc) => Ok(Some(Self{inner: acc, conn})),
            None => Ok(None)
        }
    }

    pub async fn find(conn: &'a PgPool, email: &str) -> sqlx::Result<Option<PersistentCustomer<'a>>> {
        let acc = query_as!(Customer,
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

    pub async fn get_stream(conn: &'a PgPool) -> sqlx::Result<Pin<Box<dyn Stream<Item = std::result::Result<Customer, sqlx::Error>> + 'a>>> {
        let stream = query_as!(Customer, r"SELECT id, email, salt, digest FROM customer")
            // .map(|acc| PersistentCustomer{inner: acc, conn})
            .fetch(conn);

        Ok(stream)
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
            let p = Customer::hash_password(password.as_bytes());
            let mut code = vec![0u8; 32];
            rand::thread_rng().fill(&mut code[..]); // WARN: not a crypto random generator
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

    async fn finalize_transaction(tx: &mut Transaction<'_, Postgres>, code: &[u8]) -> sqlx::Result<Option<Customer>> {
        let temp = query!(r"SELECT code, email, salt, digest FROM temp_customer WHERE code = $1", code)
            .fetch_optional(&mut *tx)
            .await?;
        
        if let Some(temp) = temp {
            let acc = query_as!(Customer,
                    r"INSERT INTO customer(email, salt, digest) VALUES ($1, $2, $3) RETURNING id, email, salt, digest",
                    &temp.email, &temp.salt, &temp.digest
                ).fetch_one(&mut *tx)
                .await?;

            query!(r"DELETE FROM temp_customer WHERE code = $1", &code)
                .execute(&mut *tx)
                .await?;

            Ok(Some(acc))
        } else {
            Ok(None)
        }
    }

    pub async fn finalize(conn: &'a PgPool, code: &[u8]) -> sqlx::Result<Option<Customer>> {
        let mut tx = conn.begin().await?;

        let result = Self::finalize_transaction(&mut tx, code).await;

        match result {
            Ok(ok) => {
                tx.commit().await?;
                Ok(ok)
            }
            Err(e) => {
                log::error!("{}", e);
                tx.rollback().await?;
                Ok(None)
            }
        }
    }

    pub async fn update_password(&'a mut self, password: &str) -> sqlx::Result<&'a mut PersistentCustomer<'a>> {
        let p = Customer::hash_password(password.as_bytes());
        let acc =  query_as!(Customer,
                r"UPDATE customer SET salt = $1, digest = $2 WHERE id = $3 RETURNING id, email, salt, digest",
                &p.salt, &p.digest, &self.inner.id
            ).fetch_one(self.conn)
            .await?;
        self.inner = acc;
        Ok(self)
    }

    pub fn into_inner(self) -> Customer {self.inner}
    pub fn inner(&self) -> &Customer {&self.inner}
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
    #[test]
    fn authentication_test() {
        let pass = "Please use a long password!".as_bytes();
        let p = Customer::hash_password(&pass);

        let cust = Customer{
            id: 123,
            email: "123@mail.com".to_owned(),
            salt: p.salt,
            digest: p.digest,
        };
        
        assert!(cust.verify_authentication(&pass));
        assert!(!cust.verify_authentication(&"Another password!".as_bytes()));
        assert!(!cust.verify_authentication(&"please use a long password!".as_bytes()));
        assert!(!cust.verify_authentication(&"Please use a long password".as_bytes()));
    }
}
