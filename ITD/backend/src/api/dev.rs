use actix_web::{HttpResponse, web, get};
use sqlx::{PgPool, query};

use crate::utils::encoding::encode_serial;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(ids);
    cfg.service(setup_env);
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


#[get("/setup_env")]
async fn setup_env(conn: web::Data<PgPool>) -> HttpResponse {
    let conn = conn.into_inner();
    
    let shops = query!(r"INSERT INTO shop (id, name, description, image, location) VALUES
        (1234111, 'Unes Milano', 'Unes via unes numero unes','test1.jpg','49.1234N,12.3456E'),
        (1234222, 'Lidl Torino', 'Lidl via lidl numero lidl','test2.jpg','123.1234N,45.3456E'),
        (1234333, 'Fruttivendolo da Attilio', 'Frutta e verdura','test3.jpg','2.1234S,23.3456W');")
        .execute(conn.as_ref())
        .await;
        
    let departments = query!(r"INSERT INTO department (id, shop_id, description, capacity) VALUES
        (4444111, 1234111, 'Frutta', 20),
        (4444222, 1234111, 'Pane', 15),
    
        (4444333, 1234222, 'Surgelati', 12),
        (4444444, 1234222, 'Carne', 20),
        (4444555, 1234222, 'Pane', 8),
    
        (4444666, 1234333, 'all', 4);")
        .execute(conn.as_ref())
        .await;

    match (shops, departments) {
        (Ok(_), Ok(_)) => HttpResponse::Ok().body("Success!"),
        _ => HttpResponse::BadRequest().body("Some query failed")
    }

}