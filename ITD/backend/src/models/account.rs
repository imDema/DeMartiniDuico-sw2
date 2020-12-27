use std::pin::Pin;

use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool, query, query_as};
use futures::Stream;

use rand::Rng;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    id: i32,
    email: String,
    salt: Vec<u8>,
    digest: Vec<u8>,
}

impl Account {
    fn argon_conf<'a>() -> argon2::Config<'a> { argon2::Config::default() }

    pub fn verify_authentication(&self, password: &[u8]) -> bool {
        let conf = Self::argon_conf();
        let matches = argon2::verify_raw(password, &self.salt, &self.digest, &conf);
        matches.unwrap_or_else(|err| {log::error!("Argon2 error!: `{:?}`", err); false})
    }

    pub fn hash_password(password: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let config = Self::argon_conf();
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill(&mut salt[..]);
        let digest = argon2::hash_raw(password, &salt, &config).expect("Argon2 error!");
        (salt, digest)
    }

    pub fn id(&self) -> i32 {self.id}
    pub fn email(&self) -> &str {&self.email}
    pub fn salt(&self) -> &[u8] {&self.salt[..]}
    pub fn digest(&self) -> &[u8] {&self.digest[..]}
}

pub struct PersistentAccount<'a> {
    inner: Account,
    conn: &'a PgPool,
}

impl<'a> PersistentAccount<'a> {
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<PersistentAccount<'a>> {
        let acc = query_as!(Account,
                r"SELECT id, email, salt, digest FROM Account WHERE id = $1",
                id
            ).fetch_one(conn)
            .await?;

        Ok(Self{inner: acc, conn})
    }

    pub async fn get_stream(conn: &'a PgPool) -> sqlx::Result<Pin<Box<dyn Stream<Item = std::result::Result<Account, sqlx::Error>> + 'a>>> {
        let stream = query_as!(Account, r"SELECT id, email, salt, digest FROM Account")
            // .map(|acc| PersistentAccount{inner: acc, conn})
            .fetch(conn);

        Ok(stream)
    }

    pub async fn create(conn: &'a PgPool, email: &str, password: &str) -> sqlx::Result<Option<PersistentAccount<'a>>> {
        let mut tx = conn.begin().await?;

        let exists = query!(r"SELECT id FROM Account WHERE email = $1", &email)
            .fetch_optional(&mut tx)
            .await?;
        
        if let None = exists {
            let (salt, digest) = Account::hash_password(password.as_bytes());
            let acc =  query_as!(Account,
                    r"INSERT INTO Account(email, salt, digest) VALUES ($1, $2, $3) RETURNING id, email, salt, digest",
                    &email, &salt, &digest
                ).fetch_one(&mut tx)
                .await?;
            tx.commit().await?;
            Ok(Some(Self{inner: acc, conn}))
        } else {
            tx.rollback().await?;
            Ok(None)
        }
    }

    pub async fn update_password(&'a mut self, password: &str) -> sqlx::Result<&'a mut PersistentAccount<'a>> {
        let (salt, digest) = Account::hash_password(password.as_bytes());
        let acc =  query_as!(Account,
                r"UPDATE Account SET salt = $1, digest = $2 WHERE id = $3 RETURNING id, email, salt, digest",
                &salt, &digest, &self.inner.id
            ).fetch_one(self.conn)
            .await?;
        self.inner = acc;
        Ok(self)
    }

    pub fn into_inner(self) -> Account {self.inner}
    pub fn inner(&self) -> &Account {&self.inner}
}