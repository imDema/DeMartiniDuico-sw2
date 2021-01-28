use actix_web::dev::{MessageBody, ServiceResponse};
use actix_web::test::TestRequest;
use actix_web::test;
use regex::Regex;
use clup::api::account::{RequestLogin, RequestRegistration};

#[macro_export]
macro_rules! req {
    ($app:expr, $req:expr) => {
        test::call_service($app, $req.to_request()).await
    }
}

pub async fn read_utf8_body<T: MessageBody + Unpin>(response: ServiceResponse<T>) -> String {
    String::from_utf8(Vec::from(&test::read_body(response).await[..])).unwrap()
}


pub fn register(email: String, password: String) -> TestRequest {
    TestRequest::post()
        .uri("/register")
        .set_json(&RequestRegistration {email, password})
}

pub fn confirm(code: String) -> TestRequest {
    TestRequest::get()
        .uri(&format!("/register/confirm?code={}", code))
}

pub fn login(email: String, password: String, remember: Option<bool>) -> TestRequest {
    TestRequest::post()
        .uri("/login")
        .set_json(&RequestLogin {email, password, remember})
}

pub fn whoami() -> TestRequest {
    TestRequest::get()
        .uri("/whoami")
}

pub fn extract_session_cookie(cookies: &str) -> Option<&str> {
    lazy_static::lazy_static!(
        static ref RE: Regex = Regex::new("actix-session=[^;]+").unwrap();
    );
    RE.captures(cookies)
        .and_then(|caps| caps.get(0))
        .map(|c| c.as_str())
}
