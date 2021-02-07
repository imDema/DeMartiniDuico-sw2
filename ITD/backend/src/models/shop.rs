use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use futures::StreamExt;

use sqlx::{FromRow, PgPool, query};
use sqlx::query_as;

use crate::utils::encoding::encode_serial;

/// Row structure for shop
#[allow(dead_code)]
#[derive(FromRow, Serialize)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub location: String,
}

/// Row structure for Department
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Department {
    pub uid: i32,
    shop_id: i32,
    description: String,
    capacity: i32
}
/// Response ready structure for department
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DepartmentResponse {
    pub(super) uid: String,
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

/// Response struct representing the current occupancy status of a department
#[derive(Serialize, Deserialize, Debug)]
pub struct DepartmentOccupancyResponse {
    pub department: DepartmentResponse,
    pub occupancy: i32,
}

/// Opening time slot for a shop
#[allow(dead_code)]
#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct Schedule {
    shop_id: i32,
    dow: i16,
    open: NaiveTime,
    close: NaiveTime,
}

///Response ready structure for shop
#[derive(Serialize, Deserialize, Debug)]
pub struct ShopResponse {
    pub uid: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub location: String,
    pub departments: Vec<DepartmentResponse>,
    pub weekly_schedule: Vec<Schedule>,
}
/// Data Access Object for shop
pub struct PersistentShop<'a> {
    conn: &'a PgPool,
    inner: Shop,
}

impl<'a> PersistentShop<'a> {
    /// Retrieve shop from its primary key
    pub async fn get(conn: &'a PgPool, id: i32) -> sqlx::Result<Option<PersistentShop<'a>>> {
        let q = query_as!(Shop,
            r"SELECT id, name, description, image, location FROM shop
            WHERE id = $1",
            id
        ).fetch_optional(conn)
        .await?;
        Ok(q.map(|q| Self {conn, inner: q}))
    }

    /// Retieve information about current per department occupancy
    pub async fn get_occupancy(conn: &'a PgPool, shop_id: i32) -> sqlx::Result<Vec<DepartmentOccupancyResponse>> {
        query!(r"SELECT
                    department.id as id,
                    description,
                    capacity,
                    count(ticket.id) as occupancy
                FROM department
                    LEFT JOIN ticket_department ON ticket_department.department_id = department.id
                    LEFT JOIN ticket 
                        ON ticket_department.ticket_id = ticket.id AND
                            ticket.entry IS NOT NULL AND
                            ticket.exit IS NULL
                WHERE
                    department.shop_id = $1    
                GROUP BY
                department.id, description, capacity", shop_id)
            .fetch(conn)
            .fold(Ok(Vec::new()), |acc, r| async {
                let mut acc = acc?;
                let r = r?;
                acc.push(DepartmentOccupancyResponse {
                    department: Department {
                        uid: r.id,
                        shop_id: shop_id,
                        description: r.description,
                        capacity: r.capacity
                    }.into(),
                    occupancy: r.occupancy.unwrap() as i32
                });
                Ok(acc)
            }).await
    }

    /// Retrieve information about schedule and departments, then transform into a response ready format
    pub async fn to_response(self) -> sqlx::Result<ShopResponse> {
        let sched = self.schedule().await?;
        let deps = self.departments().await?;

        Ok(ShopResponse {
            uid: encode_serial(self.inner.id),
            name: self.inner.name,
            description: self.inner.description,
            image: self.inner.image,
            location: self.inner.location,
            departments: deps
                .into_iter()
                .map(DepartmentResponse::from)
                .collect(),
            weekly_schedule: sched,
        })
    }

    /// Retrieve schedule for this shop
    pub async fn schedule(&self) -> sqlx::Result<Vec<Schedule>> {
        Ok(query_as!(Schedule,
            r"SELECT shop_id, dow, open, close FROM schedule
            WHERE shop_id = $1
            ORDER BY dow, open",
            self.inner.id
        ).fetch_all(self.conn)
        .await?)
    }

    /// Retrieve departments for this shop
    pub async fn departments(&self) -> sqlx::Result<Vec<Department>> {
        Ok(query_as!(Department,
            r"SELECT id as uid, shop_id, description, capacity FROM department
            WHERE shop_id = $1",
            self.inner.id
        ).fetch_all(self.conn)
        .await?)
    }

    /// Search shops by name, matches case insensitive substrings
    pub async fn search(conn: &'a PgPool, query: Option<String>) -> sqlx::Result<Vec<ShopResponse>> {
        let stream = if let Some(q) = query {
            query_as!(Shop,
                r"SELECT id, name, description, image, location FROM shop
                WHERE name ILIKE '%' || $1 || '%'
                ORDER BY name",
                q
            ).fetch(conn)
        } else {
            query_as!(Shop,
                r"SELECT id, name, description, image, location FROM shop
                ORDER BY name"
            ).fetch(conn)
        };
        let res = stream
            .fold(Ok(Vec::new()), |acc, s| async {
                let mut acc = acc?;
                let ps = Self {conn, inner: s?};
                acc.push(ps.to_response().await?);
                Ok(acc)
            }).await;

        res
    }

    pub fn into_inner(self) -> Shop {self.inner}
    pub fn inner(&self) -> &Shop {&self.inner}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    use crate::models::ticket::{EnterResult, PersistentTicket};
    use crate::utils::tests::{db, del_customer, test_customer};
    use crate::{ with_test_shop};

    #[actix_rt::test]
    async fn empty_deps_occupancy_test() -> Result<(), Box<dyn Error>> {
        let conn = db().await;
        with_test_shop!(&conn, s1 [d0, d1, d2] {
            let res = PersistentShop::get_occupancy(&conn, s1)
                .await
                .unwrap();
            assert_eq!(res.len(), 3);

            let encoded: Vec<String> = vec![d0, d1, d2].into_iter().map(encode_serial).collect();
            println!("res: {:?}\nenc: {:?}", &res, &encoded);

            for r in res.iter() {
                assert_eq!(r.occupancy, 0);
                assert!(encoded.contains(&r.department.uid));
            } 
        });

        Ok(())
    }

    #[actix_rt::test]
    async fn deps_occupancy_test() -> Result<(), Box<dyn Error>> {
        let conn = db().await;
        
        let id_c1 = test_customer(&conn).await?;
        let id_c2 = test_customer(&conn).await?;

        with_test_shop!(&conn, s1 [d0, d1] {
            let d0e = encode_serial(d0);
            let d1e = encode_serial(d1);

            let t1 = PersistentTicket::try_new(&conn, id_c1, s1, vec![d0], 25).await?.unwrap();
            let t2 = PersistentTicket::try_new(&conn, id_c2, s1, vec![d0, d1], 25).await?.unwrap();

            assert_eq!(t1.try_enter().await.unwrap(), EnterResult::Entered);
            
            let res = PersistentShop::get_occupancy(&conn, s1).await.unwrap();
            assert_eq!(res.len(), 2);
            for r in res {
                match r.department.uid {
                    id if id == d0e => assert_eq!(r.occupancy, 1),
                    id if id == d1e => assert_eq!(r.occupancy, 0),
                    _ => panic!(),
                }
            }

            assert_eq!(t2.try_enter().await.unwrap(), EnterResult::Entered);
            
            let res = PersistentShop::get_occupancy(&conn, s1).await.unwrap();
            assert_eq!(res.len(), 2);
            for r in res {
                match r.department.uid {
                    id if id == d0e => assert_eq!(r.occupancy, 2),
                    id if id == d1e => assert_eq!(r.occupancy, 1),
                    _ => panic!(),
                }
            }

            assert_eq!(t1.exit().await.unwrap(), true);
            
            let res = PersistentShop::get_occupancy(&conn, s1).await.unwrap();
            assert_eq!(res.len(), 2);
            for r in res {
                match r.department.uid {
                    id if id == d0e => assert_eq!(r.occupancy, 1),
                    id if id == d1e => assert_eq!(r.occupancy, 1),
                    _ => panic!(),
                }
            }
        });

        del_customer(&conn, id_c1).await?;
        del_customer(&conn, id_c2).await?;

        Ok(())
    }
}