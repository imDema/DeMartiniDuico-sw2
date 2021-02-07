use sqlx::{PgPool, Postgres};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::*;

pub mod models;
pub mod api;
pub mod utils;
pub mod migrations;

/// ## Setup database schema
/// + Try to connect to the supplied url
/// + Create database if it doesn't exist
/// + Create connection pool
/// + Run migrations
pub async fn setup_db(conn_url: &str) -> PgPool {
    log::info!("db_conn: {}", &conn_url);
    
    let mut retry_count = 0;
    loop {
        match Postgres::database_exists(&conn_url).await {
            Ok(true) => break,
            Ok(false) => {
                Postgres::create_database(&conn_url).await.unwrap_or_else(|e| panic!("Could not create database: {}", e));
                break;
            }
            Err(e) => {
                if retry_count >= 10 {
                    panic!("Could not connect to database after 10 tries. Terminating.")
                }
                log::warn!("Could not connect to database, retrying in 2 seconds: {}", e);
                std::thread::sleep(std::time::Duration::from_secs(2));
                retry_count += 1;
            }
        }
    }

    if !Postgres::database_exists(&conn_url).await.unwrap_or_else(|e| panic!("Could not connect to postgres: {}", e)) {
        Postgres::create_database(&conn_url).await.unwrap_or_else(|e| panic!("Could not create database: {}", e));
    }
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&conn_url).await.unwrap_or_else(|e| panic!("Could not connect to postgres: {}", e));
    
    migrations::migrate(&pool).await.unwrap_or_else(|e| panic!("Failed to run migration: {}", e));
    pool
}
