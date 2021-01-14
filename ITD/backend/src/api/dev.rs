use crate::models::account::PersistentCustomer;
use crate::utils::session;

use actix_web::{web, get, HttpResponse};
use actix_session::Session;
use sqlx::PgPool;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(whoami);
}

#[get("/whoami")]
async fn whoami(conn: web::Data<PgPool>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let uid = session::get_account(&session);
    if let Some(uid) = uid {
        let acc = PersistentCustomer::get(&conn, uid).await;
        match acc {
            Ok(Some(acc)) => 
                HttpResponse::Ok().body(format!("{:?}", acc.into_inner())),
            Ok(None) => {
                log::error!("No customer exists for uid {}", uid);
                HttpResponse::InternalServerError().body(format!("{}", uid))
            },
            Err(e) => {
                log::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Ok().body("Not logged in!")
    }
}