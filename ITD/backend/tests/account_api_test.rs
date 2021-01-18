
mod common;

use std::env;
use std::fmt::Debug;

use actix_web::dev::ServiceResponse;
use actix_web::{App, http};
use actix_web::middleware::Logger;
use actix_web::http::{HeaderMap, StatusCode};
use api::account::RequestLogin;
use rand::{Rng, thread_rng};
use actix_web::test::{TestRequest};
use actix_web::test;
use actix_redis::RedisSession;

use clup::api;
use clup::api::account::RequestRegistration;
use regex::Regex;

fn register(email: String, password: String) -> TestRequest {
    TestRequest::post()
        .uri("/register")
        .set_json(&RequestRegistration {email, password})
}

fn confirm(code: String) -> TestRequest {
    TestRequest::get()
        .uri(&format!("/register/confirm?code={}", code))
}

fn login(email: String, password: String, remember: Option<bool>) -> TestRequest {
    TestRequest::post()
        .uri("/login")
        .set_json(&RequestLogin {email, password, remember})
}

fn whoami() -> TestRequest {
    TestRequest::get()
        .uri("/whoami")
}

fn extract_session_cookie(cookies: &str) -> Option<&str> {
    lazy_static::lazy_static!(
        static ref RE: Regex = Regex::new("actix-session=[^;]+").unwrap();
    );
    RE.captures(cookies)
        .and_then(|caps| caps.get(0))
        .map(|c| c.as_str())
}

#[actix_rt::test]
async fn register_test() {
    dotenv::dotenv().ok();
    let conn_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let db_pool = clup::setup_db(&conn_url).await;

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable must be set");
    let session_key = env::var("SESSION_KEY").expect("SESSION_KEY environment variable must be set");
    let key = hex::decode(session_key).expect("Invalid SESSION_KEY format. Expected hex");

    let mut app = test::init_service(App::new()
        .data(db_pool.clone())
        .wrap(RedisSession::new(&redis_url, &key))
        .wrap(Logger::default())
        .configure(api::account::endpoints)
        .configure(api::dev::endpoints)
    ).await;
    
    let (usr, pass): (u32, u32) = (thread_rng().gen(), thread_rng().gen());
    let (email, password) = (format!("{:x}@test.com", usr), format!("{:x}", pass));
    
    // Register
    let req = register(email.clone(), password.clone()).to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let code = test::read_body(resp).await;
    assert!(code.len() > 0);
    
    // Confirm
    let code = String::from_utf8(Vec::from(&code[..])).unwrap();
    
    let req = confirm(code).to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    //Login
    let req = login(email.clone(), "wrongpass".into(), None).to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);

    let req = login(email.clone(), password.clone(), None).to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers: &HeaderMap = resp.headers();
    let cookies = headers.get("Set-Cookie").unwrap();
    let session = extract_session_cookie(cookies.to_str().unwrap()).unwrap();
    
    let req = whoami().header("Cookie", session).to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
    let resp_body = String::from_utf8(Vec::from(&test::read_body(resp).await[..])).unwrap();

    assert!(resp_body.contains(&email));

    let req = whoami().to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
    let resp_body = String::from_utf8(Vec::from(&test::read_body(resp).await[..])).unwrap();

    assert!(!resp_body.contains(&email));
}
