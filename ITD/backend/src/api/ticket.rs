use std::error::Error;

use crate::models::account::PersistentCustomer;
use crate::models::shop::PersistentShop;
use crate::models::ticket::{PersistentTicket, TicketResponse};
use crate::utils::encoding::{decode_serial, decode_serial_vec, encode_serial};
use crate::utils::session;

use actix_web::{web, get, post, HttpResponse};
use actix_session::Session;
use chrono::{NaiveDateTime, Duration, Utc};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(ticket_new);
    cfg.service(tokens);
}

#[derive(Deserialize)]
struct TicketNewRequest {
    est_minutes: i32,
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
    let shop = if let Some(s) = PersistentShop::get(conn, id).await? {
        s
    } else {
        return Ok(HttpResponse::BadRequest().body("Shop does not exist"));
    };

    let ids = decode_serial_vec(req.department_ids)?;

    let tick = PersistentTicket::new(&conn, customer_id, shop.inner().id, ids, req.est_minutes)
        .await?
        .into_inner();
    Ok(HttpResponse::Ok().body(format!("Created: {}", encode_serial(tick.id))))
}

#[get("/shop/{shop_id}/ticket/queue")]
async fn ticket_queue(conn: web::Data<PgPool>, shop_id: web::Path<String>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let shop_id = if let Ok(s) = decode_serial(&shop_id.into_inner()) {
        s
    } else {
        return HttpResponse::BadRequest().body("Shop does not exist");
    };
    
    if let Some(_) = session::get_account(&session) {
        match ticket_queue_inner(&conn, shop_id).await {
            Ok(h) => h,
            Err(e) => {
                log::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
async fn ticket_queue_inner(conn: &PgPool, shop_id: i32) -> sqlx::Result<HttpResponse> {
    let queue = PersistentTicket::queue(conn, shop_id, true, true).await?;

    let (delta_t, people) = queue.into_iter()
        .fold((Duration::zero(), 0), |(dt, p), ti| {
            (dt + Duration::minutes(ti.est_minutes as i64), p + 1)
        });

    Ok(HttpResponse::Ok().json(TicketEstResponse {
        people,
        est: Utc::now().naive_utc() + delta_t
    }))
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

#[derive(Deserialize)]
struct TicketEstQuery {
    pub uid: String,
}
#[derive(Serialize, Debug)]
struct TicketEstResponse {
    pub people: u32,
    pub est: NaiveDateTime,
}
#[get("/ticket/est")]
async fn ticket_est(conn: web::Data<PgPool>, query: web::Query<TicketEstQuery>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let q = query.into_inner();
    
    let tid = if let Ok(tid) = decode_serial(&q.uid) {
        tid
    } else {
        return HttpResponse::BadRequest().body("Invalid uid in query");
    };

    if let Some(cid) = session::get_account(&session) {
        match ticket_est_inner(&conn, cid, tid).await {
            Ok(h) => h,
            Err(e) => {
                log::error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
async fn ticket_est_inner(conn: &PgPool, cid: i32, tid: i32) -> sqlx::Result<HttpResponse> {
    if let Some(t) = PersistentTicket::get(conn, tid).await? {
        let ticket = t.into_inner();
        let now = Utc::now().naive_utc();
        if !ticket.valid || !ticket.active || ticket.expiration < now || ticket.customer_id != cid {
            log::debug!("Invalid ticket:\n{:?}", ticket);
            return Ok(HttpResponse::BadRequest().body("Expired or invalid ticket"));
        }
        let queue = PersistentTicket::queue(conn, ticket.shop_id, true, true).await?;

        let mut delta_t = Duration::minutes(0);
        let mut contained = false;
        let mut people = 0;
        for ti in queue.into_iter() {
            if ti.id == tid {
                contained = true;
                break;
            }
            people += 1;
            delta_t = delta_t + Duration::minutes(ti.est_minutes as i64);
        }

        if contained {
            Ok(HttpResponse::Ok().json(TicketEstResponse {
                people,
                est: Utc::now().naive_utc() + delta_t
            }))
        } else {
            Ok(HttpResponse::BadRequest().body("Not the owner of the ticket"))
        }
    } else {
        Ok(HttpResponse::BadRequest().body("Ticket does not exist"))
    }
}