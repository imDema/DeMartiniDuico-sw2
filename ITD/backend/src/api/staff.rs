use crate::models::staff::PersistentStaff;
use crate::models::ticket::{PersistentTicket, TicketResponse, EnterResult};
use crate::models::shop::PersistentShop;
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
    cfg.service(ticket_queue);
    cfg.service(ticket_skip);
    cfg.service(whoami);
    cfg.service(status);
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
            session::set_staff_account(&session, sa.account().id(), sa.account().email(), sa.shop_id());

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

/// Show tickets currently in queue for this shop
#[get("/shop/{shop_id}/ticket/queue")]
async fn token_info(conn: web::Data<PgPool>, shop_id: web::Path<String>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let shop_id = if let Some(sess) = session::check_staff_auth(&session, &shop_id.into_inner()) {
        sess.shop_id
    } else {
        return HttpResponse::Forbidden().finish();
    };
    
    match PersistentTicket::queue(&conn, shop_id).await {
        Ok(v) => {
            let body: Vec<TicketResponse> = v.into_iter()
                .map(TicketResponse::from)
                .collect();
            HttpResponse::Ok().json(body)
        }
        Err(e) => {
            log::error!("Error retrieving ticket queue {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
struct TokenInfoQuery {
    pub uid: String,
}
/// Show available information on a token
#[get("/shop/{shop_id}/token/info")]
async fn ticket_queue(conn: web::Data<PgPool>, shop_id: web::Path<String>, query: web::Query<TokenInfoQuery>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let q = query.into_inner();
    let s = if let Some(s) = session::check_staff_auth(&session, &shop_id.into_inner()) {
        s
    } else {
        return HttpResponse::Forbidden().finish();

    };
    let ticket_id = match decode_serial(&q.uid) {
        Ok(id) => id,
        _ => return HttpResponse::BadRequest().body("Invalid token id format"),
    };

    match PersistentTicket::get(&conn, ticket_id).await {
        Ok(Some(t)) if t.inner().shop_id == s.shop_id => 
            HttpResponse::Ok().json(TicketResponse::from(t.into_inner())),
        Ok(Some(_)) =>
            HttpResponse::Forbidden().finish(),
        Ok(None) => 
            HttpResponse::Ok().json(()),
        Err(e) => {
            log::error!("Error retrieving ticket {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Get current occupancy information
#[get("/shop/{shop_id}/status")]
async fn status(conn: web::Data<PgPool>, shop_id: web::Path<String>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let s = if let Some(s) = session::check_staff_auth(&session, &shop_id.into_inner()) {
        s
    } else {
        return HttpResponse::Forbidden().finish();
    };

    if let Ok(v) = PersistentShop::get_occupancy(&conn, s.shop_id).await {
        return HttpResponse::Ok().json(v);
    }
    HttpResponse::BadRequest().finish()
}

#[derive(Serialize, Deserialize)]
pub struct LogTicketRequest {
    pub uid: String,
}
/// Try to log the entry of a token
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
        let result = ticket.try_enter().await?;
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

#[derive(Deserialize)]
struct TicketCancelRequest {
    pub uid: String
}
/// Skip and cancel a token for this shop. Intended use is skipping customers that are late.
#[post("/shop/{shop_id}/token/skip")]
async fn ticket_skip(conn: web::Data<PgPool>, body: web::Json<TicketCancelRequest>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let req = body.into_inner();
    let sess = if let Some(sess) = session::get_staff_account(&session) {
        sess
    } else {
        return HttpResponse::Forbidden().finish();
    };
    let tid = if let Ok(tid) = decode_serial(&req.uid) {
        tid
    } else {
        return HttpResponse::BadRequest().body("Invalid uid in query");
    };

    let t = PersistentTicket::get(&conn, tid).await;

    if let Ok(Some(ticket)) = t {
        if ticket.inner().shop_id == sess.shop_id {
            if let Ok(_) = ticket.cancel().await {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            }
        } else {
            HttpResponse::Forbidden().finish()
        }
    } else {
        HttpResponse::BadRequest().body("Ticket does not exist")
    }
}

#[derive(Serialize)]
struct WhoamiResponse {
    authenticated: bool,
    email: Option<String>,
    shop_id: Option<String>,
}
/// Check the session and retrieve authentication status and email
#[get("/whoami")]
async fn whoami(session: Session) -> HttpResponse {
    if let Some(sess) = session::get_staff_account(&session) {
        let body = WhoamiResponse{
                authenticated: true,
                email: Some(sess.email),
                shop_id: Some(encode_serial(sess.shop_id))
        };
        return HttpResponse::Ok().json(body)
    } else {
        HttpResponse::Ok().json(WhoamiResponse{authenticated: false, email: None, shop_id: None})
    }
}
