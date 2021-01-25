use std::error::Error;

use crate::models::account::PersistentCustomer;
use crate::models::persistence::{decode_serial, decode_serial_vec, encode_serial};
use crate::models::shop::PersistentShop;
use crate::models::ticket::{PersistentTicket, TicketResponse};
use crate::utils::session;

use actix_web::{web, get, post, HttpResponse};
use actix_session::Session;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(ticket_new);
    cfg.service(tokens);
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TicketNewRequest {
    shop_id: String,
    department_ids: Vec<String>,
}
#[post("/shop/{shop_id}/ticket/new")]
async fn ticket_new(conn: web::Data<PgPool>, shop_id: web::Path<String>, body: web::Json<TicketNewRequest>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let shop_id = shop_id.into_inner();
    let req = body.into_inner();
    let uid = if let Some(uid) = session::get_account(&session) {
        uid
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    match ticket_new_inner(&conn, uid, &shop_id, req).await {
        Ok(resp) => resp,
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}

async fn ticket_new_inner<'a>(conn: &'a PgPool, customer_id: i32, shop_id: &str, req: TicketNewRequest) -> Result<HttpResponse, Box<dyn Error>>{
    let id = decode_serial(shop_id)?;
    let shop = PersistentShop::get(conn, id).await?.unwrap(); // TODO:

    let ids = decode_serial_vec(req.department_ids)?;

    let tick = PersistentTicket::new(&conn, customer_id, shop.inner().id, ids)
        .await?
        .into_inner();
    Ok(HttpResponse::Ok().body(format!("Created: {}", encode_serial(tick.id))))
}

#[derive(Serialize)]
struct TokensResponse {
    tickets: Vec<TicketResponse>,
    booking: Vec<()>,
}
#[get("/tokens")]
async fn tokens(conn: web::Data<PgPool>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let uid = if let Some(uid) = session::get_account(&session) {
        uid
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    match tokens_inner(&conn, uid).await {
        Ok(resp) => resp,
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}
async fn tokens_inner(conn: &PgPool, uid: i32) -> sqlx::Result<HttpResponse> {
    let customer = PersistentCustomer::get(conn, uid).await?;
    if let Some(_) = customer {
        let tickets = PersistentTicket::get_for_customer(conn, uid).await?;
        let ticket_resp: Vec<TicketResponse> = tickets.into_iter()
            .map(|t|t.into())
            .collect();

        let resp = TokensResponse {
            tickets: ticket_resp,
            booking: Vec::new(),
        };

        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}