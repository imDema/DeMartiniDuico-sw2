
mod common;

use std::env;

use actix_web::{App, http};
use actix_web::middleware::Logger;
use api::account::RequestLogin;
use rand::{Rng, thread_rng};
use actix_web::test::{TestRequest};
use actix_web::test;
use actix_redis::RedisSession;

use clup::api;
use clup::api::account::RequestRegistration;

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

    // Register
    let (usr, pass): (u32, u32) = (thread_rng().gen(), thread_rng().gen());
    let (email, password) = (format!("{:x}@test.com", usr), format!("{:x}", pass));

    let req = TestRequest::post()
        .uri("/register")
        .set_json(&RequestRegistration {email, password})
        .to_request();

    println!("{:?}", &req);
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    println!("{:?}", &resp);

    // Confirm
    let code = test::read_body(resp).await;
    assert!(code.len() > 0);

    let code = String::from_utf8(Vec::from(&code[..])).unwrap();
    
    let req = TestRequest::get()
        .uri(&format!("/register/confirm?code={}", code))
        .to_request();

    println!("{:?}", &req);
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    println!("{:?}", &resp);

    //Login
    let (email, password) = (format!("{:x}@test.com", usr), format!("{:x}", pass));

    let req = TestRequest::post()
        .uri("/login")
        .set_json(&RequestLogin {email, password, remember: None})
        .to_request();

    println!("{:?}", &req);
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    println!("{:?}", &resp);

}
