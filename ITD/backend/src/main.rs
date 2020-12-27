use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};
use clup::models::account::PersistentAccount;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use futures::TryStreamExt;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"Welcome to Clup
    __       __
    / <`     '> \
   (  / @   @ \  )
    \(_ _\_/_ _)/
  (\ `-/     \-' /)
   "===\     /==="
    .==')___(`==.
   ' .='     `=."#)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let pg_host =   env::var("PG_HOST").expect("PG_HOST environment variable must be set");
    let pg_db =     env::var("PG_DB").expect("PG_DB environment variable must be set");
    let pg_user =   env::var("PG_USER").expect("PG_USER environment variable must be set");
    let pg_pass =   env::var("PG_PASSWORD").expect("PG_PASSWORD environment variable must be set");

    let conn_url = format!("postgres://{}:{}@{}/{}", &pg_user, &pg_pass, &pg_host, &pg_db);
    log::debug!("db_conn: {}", &conn_url);

    let db_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&conn_url).await.expect("Could not connect to database");

    let api_url = env::var("API_URL").unwrap_or("0.0.0.0:5000".into());

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(index)
            .service(accounts)
            // .route("/", web::get().to(index))
            // .service(web::scope("/api").configure(api::endpoints))
    })
    .bind(api_url)?
    .run()
    .await
}