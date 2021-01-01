use crate::models::account::PersistentAccount;

use actix_web::{web, get, post, Responder, HttpResponse};
use sqlx::PgPool;
use serde::Deserialize;
use futures::TryStreamExt;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(login);
    cfg.service(register);
    cfg.service(accounts);
}

#[get("/")]
async fn index() -> impl Responder {
    let template = include_str!("../../templates/login.html");

    HttpResponse::Ok().body(template)
}

#[derive(Deserialize)]
struct FormAccess {
    email: String,
    password: String,
    remember: bool,
}

#[post("/login")]
async fn login(conn: web::Data<PgPool>, form: web::Form<FormAccess>) -> HttpResponse {
    let conn = conn.into_inner();
    let error = HttpResponse::Unauthorized().body("Invalid email or password");
    let acc = PersistentAccount::find(&conn, &form.email).await;
    if let Ok(Some(acc)) = acc {
        let acc = acc.into_inner();
        if acc.verify_authentication(form.password.as_bytes()) {
            HttpResponse::Found().set_header("Location", "/accounts").finish()
        } else {
            error
        }
    } else {
        error
    }
}

#[post("/register")]
async fn register(conn: web::Data<PgPool>, form: web::Form<FormAccess>) -> HttpResponse {
    let conn = conn.into_inner();
    let acc = PersistentAccount::create(&conn, &form.email, &form.password).await;
    if let Ok(res) = acc {
        if let Some(_acc) = res {
            HttpResponse::Found().set_header("Location", "/").finish()
        } else {
            HttpResponse::Forbidden().body("Forbidden") // TODO
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/accounts")]
async fn accounts(conn: web::Data<PgPool>) -> impl Responder {
    if let Ok(mut stream) = PersistentAccount::get_stream(&conn.into_inner()).await {
        let mut body = String::new();
        while let Ok(Some(acc)) = stream.try_next().await {
            body.push_str(&format!("{:?}", acc));
            body.push('\n');
        }
        HttpResponse::Ok().body(&body)
    } else {
        HttpResponse::InternalServerError().body("Internal server error")
    }
}

