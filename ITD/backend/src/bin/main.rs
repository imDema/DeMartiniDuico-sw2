use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use actix_redis::RedisSession;
use actix_cors::Cors;
use clup::api;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let conn_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let db_pool = clup::setup_db(&conn_url).await;

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable must be set");
    let key = session_key();

    let api_url = env::var("API_URL").unwrap_or("0.0.0.0:5000".into());
    HttpServer::new(move || {
        let cors = Cors::default() // Dev purposes
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
        .wrap(Logger::default())
        .wrap(RedisSession::new(&redis_url, &key)
                    .cookie_same_site(actix_redis::SameSite::Lax) // Dev purposes
                    // .cookie_secure(true) // Commented out for the prototype, production would have secure cookies
                    .ttl(604800))
        .wrap(cors)
        .data(db_pool.clone())
        .configure(api::account::endpoints)
        .configure(api::ticket::endpoints)
        .configure(api::shop::endpoints)
        .service(web::scope("/staff").configure(api::staff::endpoints))
        .service(web::scope("/dev").configure(api::dev::endpoints))
    })
    .bind(api_url)?
    .run()
    .await
}

/// For testing purposes this provides a default, it shouldn't in production
fn session_key() -> Vec<u8> {
    match env::var("SESSION_KEY") {
        Ok(a) if a.len() > 0 => hex::decode(a).expect("Invalid SESSION_KEY format. Expected hex"),
        _ => {
            println!("WARNING USING DEFAULT SESSION KEY, THIS IS ENABLED ONLY FOR TESTING PURPOSES, DO NOT USE IN PRODUCTION");
            vec![0; 32]
        }
    }
}