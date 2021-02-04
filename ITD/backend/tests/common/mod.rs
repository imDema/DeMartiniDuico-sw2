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
            .configure(api::shop::endpoints)
            .service(actix_web::web::scope("/staff").configure(api::staff::endpoints))
            .service(actix_web::web::scope("/dev").configure(api::dev::endpoints))
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

        let r = req!(whoami(), &session, $app);
        let r_body = read_utf8_body(r).await;
        assert!(r_body.contains(&email));

        (email, password, session)
    }};
}

#[macro_export]
macro_rules! quick_create_staff {
    ($app:expr, $shop_id:expr) => {{
        use rand::{RngCore, thread_rng};
        let (email, password) = (format!("{:x}@test.com", thread_rng().next_u64()), format!("{:x}", thread_rng().next_u64()));
        let r = req!(create_staff(&email, &password, $shop_id), $app);
        assert_eq!(r.status(), actix_web::http::StatusCode::OK);
        let r = req!(staff_login(&email, &password, None), $app);
        assert_eq!(r.status(), actix_web::http::StatusCode::OK);
        let cookies = r.headers().get("Set-Cookie").unwrap();
        let session = common::extract_session_cookie(cookies.to_str().unwrap()).unwrap().to_owned();

        let r = req!(whoami(), &session, $app);
        let r_body = read_utf8_body(r).await;
        assert!(r_body.contains(&email));

        (email, password, session)
    }};
}

#[macro_export]
macro_rules! ticket {
    ($shop:expr, [$($did:expr),+], $est:expr, $cookies:expr, $app:expr) => {{
        let dids = vec![$($did.as_str(), )+];
        let r = req!(ticket_new($shop, &dids[..], $est), $cookies, $app);
        assert_eq!(r.status(), StatusCode::OK);

        let t: TicketResponse = test::read_body_json(r).await;
        assert_eq!($shop, &t.shop_id);
        assert_eq!(t.department_ids.len(), dids.len());
        $(
        assert!(t.department_ids.contains($did));
        )+
        assert!(t.valid);
        assert!(t.active);
        t
    }}
}


pub fn extract_session_cookie(cookies: &str) -> Option<&str> {
    lazy_static::lazy_static!(
        static ref RE: Regex = Regex::new("actix-session=[^;]+").unwrap();
    );
    RE.captures(cookies)
        .and_then(|caps| caps.get(0))
        .map(|c| c.as_str())
}