pub mod requests;

use regex::Regex;

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

#[macro_export]
macro_rules! quick_create_customer {
    ($app:expr) => {{
        use rand::{RngCore, thread_rng};
        let (email, password) = (format!("{:x}@test.com", thread_rng().next_u64()), format!("{:x}", thread_rng().next_u64()));
        let r = req!(register(&email, &password), $app);
        let code = read_utf8_body(r).await;
        let r = req!(confirm(&code), $app);
        assert_eq!(r.status(), actix_web::http::StatusCode::OK);
        let r = req!(login(&email, &password, None), $app);
        assert_eq!(r.status(), actix_web::http::StatusCode::OK);
        let cookies = r.headers().get("Set-Cookie").unwrap();
        let session = common::extract_session_cookie(cookies.to_str().unwrap()).unwrap().to_owned();
        (email, password, session)
    }};
}

pub fn extract_session_cookie(cookies: &str) -> Option<&str> {
    lazy_static::lazy_static!(
        static ref RE: Regex = Regex::new("actix-session=[^;]+").unwrap();
    );
    RE.captures(cookies)
        .and_then(|caps| caps.get(0))
        .map(|c| c.as_str())
}
