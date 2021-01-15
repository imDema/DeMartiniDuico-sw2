use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use actix_redis::RedisSession;
use clup::api;

use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_pool = clup::setup_db().await;
    
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable must be set");
    let session_key = env::var("SESSION_KEY").expect("SESSION_KEY environment variable must be set");
    let key = hex::decode(session_key).expect("Invalid SESSION_KEY format. Expected hex");

    let api_url = env::var("API_URL").unwrap_or("0.0.0.0:5000".into());
    
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(RedisSession::new(&redis_url, &key))
        .data(db_pool.clone())
        .configure(api::account::endpoints)
        .service(web::scope("/dev").configure(api::dev::endpoints))
    })
    .bind(api_url)?
    .run()
    .await
}