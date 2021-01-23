use std::error::Error;

use crate::models::persistence::{decode_serial, encode_serial};
use crate::models::shop::PersistentShop;
use crate::utils::session;

use actix_web::{web, get, post, HttpResponse};
use actix_session::Session;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(ticket_new);
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

    let ids = req.department_ids.iter()
        .map(|id| decode_serial(id))
        .collect::<Result<Vec<i32>,_>>()?;


    let tick = shop.new_ticket(customer_id, ids).await?;
    match tick {
        Some(t) => Ok(HttpResponse::Ok().body(format!("Created: {}", encode_serial(t.id)))),
        _ => Ok(HttpResponse::InternalServerError().finish()),
    }
}