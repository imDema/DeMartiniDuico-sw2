
mod common;
use common::{extract_session_cookie, requests::*};

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
    let resp = req!(register(&email, &password), &mut app);
    assert_eq!(resp.status(), StatusCode::OK);

    let code = test::read_body(resp).await;
    assert!(code.len() > 0);
    
    // Confirm
    let code = String::from_utf8(Vec::from(&code[..])).unwrap();
    
    let resp = req!(confirm(&code), &mut app);
    assert_eq!(resp.status(), http::StatusCode::OK);

    //Login
    let resp = req!(login(&email, "wrongpass".into(), None), &mut app);
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

    let resp = req!(login(&email, &password, None), &mut app);
    assert_eq!(resp.status(), http::StatusCode::OK);

    let headers: &HeaderMap = resp.headers();
    let cookies = headers.get("Set-Cookie").unwrap();
    let session = extract_session_cookie(cookies.to_str().unwrap()).unwrap();
    
    let resp = req!(whoami(), session, &mut app);
    assert_eq!(resp.status(), http::StatusCode::OK);

    let resp_body = read_utf8_body(resp).await;
    assert!(resp_body.contains(&email));

    let resp = req!(whoami(), &mut app);
    assert_eq!(resp.status(), http::StatusCode::OK);

    let resp_body = read_utf8_body(resp).await;
    assert!(!resp_body.contains(&email));
}
