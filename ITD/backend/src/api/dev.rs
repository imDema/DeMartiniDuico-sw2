use actix_web::{HttpResponse, web, get};
use sqlx::{PgPool, query};
use serde::{Serialize, Deserialize};

use crate::models::staff::PersistentStaff;
use crate::utils::encoding::{decode_serial, encode_serial};

/// # WARNING: These endpoints should not be active in production
/// Development endpoints
pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(ids);
    cfg.service(new_staff);
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
        (1234333, 'Fruttivendolo da Attilio', 'Frutta e verdura','test3.jpg','2.1234S,23.3456W'),
        (1234444, 'Casa dolce casa', 'Tutto per la casa','test4.jpg','46.1234S,23.3456W'),
        (1234555, 'Green market sas', 'Frutta e verdura per tutti i gusti','test5.jpg','23.1234S,23.3456W'),
        (1234666, 'ParmaTop Salumeria', 'La miglior mortadella di Parma','test6.jpg','5.1234S,123.3456E');")
        .execute(conn.as_ref())
        .await;
        
    let departments = query!(r"INSERT INTO department (shop_id, description, capacity) VALUES
        (1234111, 'Frutta', 20),
        (1234111, 'Pane', 15),
    
        (1234222, 'Surgelati', 12),
        (1234222, 'Carne', 20),
        (1234222, 'Pane', 2),
        
        (1234333, 'all', 4),
        
        (1234444, 'Prodotti per il bagno', 12),
        (1234444, 'Prodotti per la cucina', 20),
        (1234444, 'Giardinaggio', 2),
            
        (1234555, 'Frutta', 12),
        (1234555, 'Verdura', 20),
        (1234555, 'Pane', 8),
        (1234555, 'Latticini', 8),

        (1234666, 'Insaccati', 12),
        (1234666, 'Carne', 20),
        (1234666, 'Formaggi', 14);")
        .execute(conn.as_ref())
        .await;

    let sched = query!(r"INSERT INTO schedule (shop_id, dow, open, close) VALUES
        (1234111, 1, '09:00', '17:00'),
        (1234111, 2, '09:00', '17:00'),
        (1234111, 3, '09:00', '17:00'),
        (1234111, 4, '09:00', '17:00'),
        (1234111, 5, '09:00', '17:00'),

        (1234222, 1, '09:00', '17:00'),
        (1234222, 2, '09:00', '17:00'),
        (1234222, 3, '09:00', '17:00'),
        (1234222, 4, '09:00', '17:00'),
        (1234222, 5, '09:00', '17:00'),

        (1234333, 1, '09:00', '17:00'),
        (1234333, 2, '09:00', '17:00'),
        (1234333, 3, '09:00', '17:00'),
        (1234333, 4, '09:00', '17:00'),
        (1234333, 5, '09:00', '17:00'),

        (1234444, 1, '09:00', '17:00'),
        (1234444, 2, '09:00', '17:00'),
        (1234444, 3, '09:00', '17:00'),
        (1234444, 4, '09:00', '17:00'),
        (1234444, 5, '09:00', '17:00'),

        (1234555, 1, '09:00', '17:00'),
        (1234555, 2, '09:00', '17:00'),
        (1234555, 3, '09:00', '17:00'),
        (1234555, 4, '09:00', '17:00'),
        (1234555, 5, '09:00', '17:00'),

        (1234666, 1, '09:00', '17:00'),
        (1234666, 2, '09:00', '17:00'),
        (1234666, 3, '09:00', '17:00'),
        (1234666, 4, '09:00', '17:00'),
        (1234666, 5, '09:00', '17:00');")    
        .execute(conn.as_ref())
        .await;

    match (shops, departments, sched) {
        (Ok(_), Ok(_), Ok(_)) => HttpResponse::Ok().body("Success!"),
        _ => HttpResponse::BadRequest().body("Some query failed")
    }
}

#[derive(Serialize, Deserialize)]
struct NewStaffQuery {
    pub email: String,
    pub password: String,
    pub shop_id: String,
}

/// ### Create a new staff account
/// GET method is easier to use in tests and the endpoint **should not be active** in production anyway
/// so security isn't a concern 
#[get("/new_staff")]
async fn new_staff(conn: web::Data<PgPool>, query: web::Query<NewStaffQuery>) -> HttpResponse {
    let conn = conn.into_inner();
    let q = query.into_inner();
    let shop_id = match decode_serial(&q.shop_id) {
        Ok(s) => s,
        _ => return HttpResponse::BadRequest().body("Invalid shop id format"),
    };

    match PersistentStaff::create(&conn, &q.email, &q.password, shop_id).await {
        Ok(Some(_)) =>
            HttpResponse::Ok().body(&format!(r#"Created staff for "{}" with email "{}" and password "{}""#, q.shop_id, q.email, q.password)),
        Ok(None) => 
            HttpResponse::Ok().body("A staff account with the same email already exists!"),
        Err(e) => {
            log::error!("Error in staff creation {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
