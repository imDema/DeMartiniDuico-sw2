use crate::models::staff::PersistentStaff;
use crate::utils::session;

use actix_web::{web, get, post, HttpResponse};
use actix_session::Session;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(logout);
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

// Token info

// log-entry

// log-exit