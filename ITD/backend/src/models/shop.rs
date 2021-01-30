use serde::{Serialize, Deserialize};

use chrono::prelude::*;

use sqlx::{FromRow, PgPool};
use sqlx::query_as;

use crate::utils::encoding::encode_serial;

#[allow(dead_code)]
#[derive(FromRow, Serialize)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub location: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Department {
    uid: i32,
    shop_id: i32,
    description: String,
    capacity: i32
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DepartmentResponse {
    uid: String,
    description: String,
    capacity: i32,
}

impl From<Department> for DepartmentResponse {
    fn from(d: Department) -> Self {
        Self {
            uid: encode_serial(d.uid),
            description: d.description,
            capacity: d.capacity,
        }
    }
}

#[allow(dead_code)]
#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct Schedule {
    shop_id: i32,
    dow: i16,
    open: NaiveTime,
    close: NaiveTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShopResponse {
    pub uid: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub location: String,
    pub departments: Vec<DepartmentResponse>,
    pub weekly_schedule: Vec<Schedule>,
}
pub struct PersistentShop<'a> {
    conn: &'a PgPool,
    inner: Shop,
}

impl<'a> PersistentShop<'a> {
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentShop<'a>>> {
        let q = query_as!(Shop,
            r"SELECT id, name, description, image, location FROM shop
            WHERE id = $1",
            id
        ).fetch_optional(conn)
        .await?;
        Ok(q.map(|q| Self {conn, inner: q}))
    }

    pub async fn to_response(self) -> sqlx::Result<ShopResponse> {
        let sched = self.schedule().await?;
        let deps = self.departments().await?;

        Ok(ShopResponse {
            uid: self.inner.id,
            name: self.inner.name,
            description: self.inner.description,
            image: self.inner.image,
            location: self.inner.location,
            departments: deps
                .into_iter()
                .map(|dep| DepartmentResponse::from(dep))
                .collect(),
            weekly_schedule: sched,
        })
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
            r"SELECT id as uid, shop_id, description, capacity FROM department
            WHERE shop_id = $1",
            self.inner.id
        ).fetch_all(self.conn)
        .await?)
    }

    pub fn into_inner(self) -> Shop {self.inner}
    pub fn inner(&self) -> &Shop {&self.inner}
}
