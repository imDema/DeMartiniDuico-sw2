
mod common;
use common::requests::*;

use actix_web::http;
use actix_web::http::{HeaderMap, StatusCode};
use rand::{Rng, thread_rng};
use actix_web::test;

#[actix_rt::test]
async fn register_test() {
    let mut app = setup_app!();
    
    let (usr, pass): (u32, u32) = (thread_rng().gen(), thread_rng().gen());
    let (email, password) = (format!("{:x}@test.com", usr), format!("{:x}", pass));
    
    // Register
    let resp = req!(&mut app, register(email.clone(), password.clone()));
    assert_eq!(resp.status(), StatusCode::OK);

    let code = test::read_body(resp).await;
    assert!(code.len() > 0);
    
    // Confirm
    let code = String::from_utf8(Vec::from(&code[..])).unwrap();
    
    let resp = req!(&mut app, confirm(code));
    assert_eq!(resp.status(), http::StatusCode::OK);

    //Login
    let resp = req!(&mut app, login(email.clone(), "wrongpass".into(), None));
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);

    let resp = req!(&mut app, login(email.clone(), password.clone(), None));
    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers: &HeaderMap = resp.headers();
    let cookies = headers.get("Set-Cookie").unwrap();
    let session = extract_session_cookie(cookies.to_str().unwrap()).unwrap();
    
    let resp = req!(&mut app, whoami().header("Cookie", session));
    assert_eq!(resp.status(), http::StatusCode::OK);

    let resp_body = read_utf8_body(resp).await;
    assert!(resp_body.contains(&email));

    let resp = req!(&mut app, whoami());
    assert_eq!(resp.status(), http::StatusCode::OK);

    let resp_body = read_utf8_body(resp).await;
    assert!(!resp_body.contains(&email));
}
