use actix_web::{HttpResponse, web, get};
use sqlx::{PgPool, query};

use crate::models::persistence::encode_serial;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(ids);
}


#[get("/ids")]
async fn ids(conn: web::Data<PgPool>) -> HttpResponse {
    let conn = conn.into_inner();

    let mut body = String::new();
    
    let shops = query!(r"SELECT id FROM shop")
        .fetch_all(&*conn)
        .await.unwrap();

    let departments = query!(r"SELECT id, shop_id FROM department")
        .fetch_all(&*conn)
        .await.unwrap();

    let customers = query!(r"SELECT id FROM customer")
        .fetch_all(&*conn)
        .await.unwrap();

    let tickets = query!(r"SELECT id, customer_id FROM ticket")
        .fetch_all(&*conn)
        .await.unwrap();

    body.push_str("Shop:\n");
    for row in shops {
        body.push_str(&format!("id: {}\n", encode_serial(row.id)));
    }
    body.push_str("\nDepartments:\n");
    for row in departments {
        body.push_str(&format!("id: {}, shop_id: {}\n", encode_serial(row.id), encode_serial(row.shop_id)));
    }
    body.push_str("\nCustomers:\n");
    for row in customers {
        body.push_str(&format!("id: {}\n", encode_serial(row.id)));
    }
    body.push_str("\nTickets:\n");
    for row in tickets {
        body.push_str(&format!("id: {}, customer_id: {}\n", encode_serial(row.id), encode_serial(row.customer_id)));
    }

    HttpResponse::Ok().body(body)
}