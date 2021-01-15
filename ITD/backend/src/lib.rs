use sqlx::{PgPool, Postgres};
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::*;

use std::env;

pub mod models;
pub mod api;
pub mod utils;
pub mod migrations;


pub async fn setup_db() -> PgPool {
    let conn_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    log::info!("db_conn: {}", &conn_url);
    
    if !Postgres::database_exists(&conn_url).await.unwrap_or_else(|e| panic!("Could not connect to postgres: {}", e)) {
        Postgres::create_database(&conn_url).await.unwrap_or_else(|e| panic!("Could not create database: {}", e));
    }
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&conn_url).await.unwrap_or_else(|e| panic!("Could not connect to postgres: {}", e));
    
    migrations::migrate(&pool).await.unwrap_or_else(|e| panic!("Failed to run migration; {}", e));
    pool
}