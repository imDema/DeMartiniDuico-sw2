use sqlx::{FromRow, PgPool, query, query_as};

use super::account::Account;

/// Internal staff structure, wraps [`Account`] adding a shop id
pub struct Staff {
    account: Account,
    shop_id: i32,
}

impl Staff {
    pub fn shop_id(&self) -> i32 { self.shop_id }
    /// Get inner account structure
    pub fn account(&self) -> &Account { &self.account }
}

impl From<StaffRow> for Staff {
    fn from(row: StaffRow) -> Self {
        let account = Account {
            id: row.id,
            email: row.email,
            salt: row.salt,
            digest: row.digest,
        };

        Self {
            account,
            shop_id: row.shop_id,
        }
    }
}

/// Row structure for staff
#[derive(FromRow)]
struct StaffRow {
    id: i32,
    shop_id: i32,
    email: String,
    salt: Vec<u8>,
    digest: Vec<u8>,
}

/// Data Access Object for staff
#[allow(dead_code)]
pub struct PersistentStaff<'a> {
    inner: Staff,
    conn: &'a PgPool,
}

impl<'a> PersistentStaff<'a> {
    /// Retrieve staff from its primary key
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentStaff<'a>>> {
        let acc = query_as!(StaffRow,
            r"SELECT id, email, salt, digest, shop_id FROM staff WHERE id = $1",
            id
        ).fetch_optional(conn)
        .await?;

        Ok(acc.map(|acc|Self{inner: acc.into(), conn}))
    }

    /// Retrieve staff from its email
    pub async fn find(conn: &'a PgPool, email: &str) -> sqlx::Result<Option<PersistentStaff<'a>>> {
        let acc = query_as!(StaffRow,
                r"SELECT id, email, salt, digest, shop_id FROM staff WHERE email = $1",
                email
            ).fetch_optional(conn)
            .await?;
        if let Some(acc) = acc {
            Ok(Some(Self{inner: acc.into(), conn}))
        } else {
            Ok(None)
        }
    }

    /// Create a new staff account (for development purposes there are no confirmation steps)
    pub async fn create(conn: &'a PgPool, email: &str, password: &str, shop_id: i32) -> sqlx::Result<Option<PersistentStaff<'a>>> {
        let mut tx = conn.begin().await?;

        let exists = query!(r"SELECT email FROM staff WHERE email = $1", &email)
            .fetch_optional(&mut tx)
            .await?;
        
        if let None = exists {
            let p = Account::hash_password(password.as_bytes());
            let acc =  query_as!(StaffRow,
                    r"INSERT INTO staff (shop_id, email, salt, digest)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id, email, salt, digest, shop_id",
                    shop_id, &email, &p.salt, &p.digest
                ).fetch_one(&mut tx)
                .await?;
            tx.commit().await?;
            Ok(Some(PersistentStaff{conn, inner: acc.into()}))
        } else {
            tx.rollback().await?;
            Ok(None)
        }
    }

    pub fn into_inner(self) -> Staff {self.inner}
    pub fn inner(&self) -> &Staff {&self.inner}
}
