
use crate::models::shop::PersistentShop;
use crate::utils::encoding::decode_serial;
use crate::utils::session;

use actix_web::{web, get, HttpResponse};
use actix_session::Session;
use sqlx::PgPool;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(shop_info);
}

#[get("/shop/{shop_id}")]
async fn shop_info(conn: web::Data<PgPool>, shop_id: web::Path<String>, session: Session) -> HttpResponse {
    let conn = conn.into_inner();
    let shop_id = if let Ok(s) = decode_serial(&shop_id.into_inner()) {
        s
    } else {
        return HttpResponse::BadRequest().body("Invalid shop id format")
    };
    if let None = session::get_account(&session) {
        return HttpResponse::Forbidden().finish();
    }

    if let Ok(Some(s)) = PersistentShop::get(&conn, shop_id).await {
        if let Ok(resp) = s.to_response().await {
            return HttpResponse::Ok().json(resp);
        }
    }
    HttpResponse::BadRequest().finish()
}
