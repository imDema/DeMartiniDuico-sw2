pub mod requests;

#[macro_export]
macro_rules! setup_app {
    () => {{
        use std::env;
        use clup::api;
        dotenv::dotenv().ok();
        let conn_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        let db_pool = clup::setup_db(&conn_url).await;

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable must be set");
        let session_key = env::var("SESSION_KEY").expect("SESSION_KEY environment variable must be set");
        let key = hex::decode(session_key).expect("Invalid SESSION_KEY format. Expected hex");

        actix_web::test::init_service(actix_web::App::new()
            .data(db_pool.clone())
            .wrap(actix_redis::RedisSession::new(&redis_url, &key))
            .wrap(actix_web::middleware::Logger::default())
            .configure(api::account::endpoints)
            .configure(api::ticket::endpoints)
            .configure(api::dev::endpoints)
        ).await
    }}
}
