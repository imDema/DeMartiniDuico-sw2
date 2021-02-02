use crate::models::staff::PersistentStaff;
use crate::models::ticket::{PersistentTicket, TicketResponse, EnterResult};
use crate::utils::encoding::{decode_serial, encode_serial};
use crate::utils::session;

use actix_web::{web, get, post, HttpResponse};
use actix_session::Session;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(logout);
    cfg.service(token_info);
    cfg.service(log_entry);
    cfg.service(log_exit);
}
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestLogin {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

#[post("/login")]
async fn login(conn: web::Data<PgPool>, body: web::Json<RequestLogin>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let req = body.into_inner();
    let error = HttpResponse::BadRequest().body("Invalid email or password");
    let staff_acc = PersistentStaff::find(&conn, &req.email).await;
    
    if let Ok(Some(staff_acc)) = staff_acc {
        let sa = staff_acc.into_inner();
        if sa.account().verify_authentication(req.password.as_bytes()) {
            session::set_staff_account(&session, sa.account().id(), sa.shop_id());

            // session.renew();
            HttpResponse::Ok().finish()
        } else {
            log::debug!("Invalid password");
            error
        }
    } else {
        log::debug!("Account does not exist");
        error
    }
}

#[get("/logout")]
async fn logout(_conn: web::Data<PgPool>, session: Session) -> HttpResponse {
    // let conn = conn.into_inner();
    session::clear_staff_account(&session);
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct TokenInfoQuery {
    pub uid: String,
}
#[get("/shop/{shop_id}/token/info")]
async fn token_info(conn: web::Data<PgPool>, shop_id: web::Path<String>, query: web::Query<TokenInfoQuery>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let q = query.into_inner();
    if let None = session::check_staff_auth(&session, &shop_id.into_inner()) {
        return HttpResponse::Forbidden().finish();
    }
    let ticket_id = match decode_serial(&q.uid) {
        Ok(id) => id,
        _ => return HttpResponse::BadRequest().body("Invalid token id format"),
    };

    match PersistentTicket::get(&conn, ticket_id).await {
        Ok(Some(t)) => 
            HttpResponse::Ok().json(TicketResponse::from(t.into_inner())),
        Ok(None) => 
            HttpResponse::Ok().json(()),
        Err(e) => {
            log::error!("Error retrieving ticket {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogTicketRequest {
    pub uid: String,
}
#[post("/shop/{shop_id}/token/log-entry")]
async fn log_entry(conn: web::Data<PgPool>, shop_id: web::Path<String>, query: web::Json<LogTicketRequest>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let q = query.into_inner();
    if let None = session::check_staff_auth(&session, &shop_id.into_inner()) {
        return HttpResponse::Forbidden().finish();
    }
    let ticket_id = match decode_serial(&q.uid) {
        Ok(id) => id,
        _ => return HttpResponse::BadRequest().body("Invalid token id format"),
    };
    
    match log_entry_inner(&conn, ticket_id).await {
        Ok(resp) => resp,
        Err(e) => {
            log::error!("Error logging entry: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
async fn log_entry_inner(conn: &PgPool, ticket_id: i32) -> sqlx::Result<HttpResponse> {
    if let Some(ticket) = PersistentTicket::get(conn, ticket_id).await? {
        let result = ticket.enter().await?;
        match result {
            EnterResult::Entered => Ok(HttpResponse::Ok().finish()),
            EnterResult::Full(did) => Ok(HttpResponse::BadRequest().body(&format!("Department {} is full", encode_serial(did)))),
            EnterResult::NotFirst(n) => Ok(HttpResponse::BadRequest().body(&format!("Not first in line, {} ahead", n))),
            EnterResult::Expired => Ok(HttpResponse::BadRequest().body("Expired")),
            EnterResult::Invalid => Ok(HttpResponse::BadRequest().body("Invalid"))
        }
    } else {
        Ok(HttpResponse::BadRequest().body("Ticket does not exist"))
    }
}

#[post("/shop/{shop_id}/token/log-exit")]
async fn log_exit(conn: web::Data<PgPool>, shop_id: web::Path<String>, query: web::Json<LogTicketRequest>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let q = query.into_inner();
    if let None = session::check_staff_auth(&session, &shop_id.into_inner()) {
        return HttpResponse::Forbidden().finish();
    }
    let ticket_id = match decode_serial(&q.uid) {
        Ok(id) => id,
        _ => return HttpResponse::BadRequest().body("Invalid token id format"),
    };
    
    match log_exit_inner(&conn, ticket_id).await {
        Ok(resp) => resp,
        Err(e) => {
            log::error!("Error logging exit: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
async fn log_exit_inner(conn: &PgPool, ticket_id: i32) -> sqlx::Result<HttpResponse> {
    if let Some(ticket) = PersistentTicket::get(conn, ticket_id).await? {
        let success = ticket.exit().await?;
        if success {
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::BadRequest().finish())
        }
    } else {
        Ok(HttpResponse::BadRequest().body("Ticket does not exist"))
    }
}
