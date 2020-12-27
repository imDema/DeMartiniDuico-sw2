use actix_web::{ web, App, HttpServer, Responder, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use std::env;

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
            .route("/", web::get().to(index))
            // .service(web::scope("/api").configure(api::endpoints))
    })
    .bind(api_url)?
    .run()
    .await
}