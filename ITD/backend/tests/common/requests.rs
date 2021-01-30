
use actix_web::dev::{MessageBody, ServiceResponse};
use actix_web::test::TestRequest;
use actix_web::test;
use clup::api::ticket::TicketNewRequest;
use clup::api::account::{RequestLogin, RequestRegistration};

#[macro_export]
macro_rules! req {
    ($req:expr, $app:expr) => {
        test::call_service($app, $req.to_request()).await
    };
    ($req:expr, $cookies:expr, $app:expr) => {
        test::call_service($app, $req.header("Cookie", String::from($cookies)).to_request()).await
    }
}

pub async fn read_utf8_body<T: MessageBody + Unpin>(response: ServiceResponse<T>) -> String {
    String::from_utf8(Vec::from(&test::read_body(response).await[..])).unwrap()
}


pub fn register(email: &str, password: &str) -> TestRequest {
    TestRequest::post()
        .uri("/register")
        .set_json(&RequestRegistration {
            email: email.to_owned(),
            password: password.to_owned()
        })
}

pub fn confirm(code: &str) -> TestRequest {
    TestRequest::get()
        .uri(&format!("/register/confirm?code={}", code))
}

pub fn login(email: &str, password: &str, remember: Option<bool>) -> TestRequest {
    TestRequest::post()
        .uri("/login")
        .set_json(&RequestLogin {
            email: email.to_owned(), 
            password: password.to_owned(), 
            remember
        })
}

pub fn whoami() -> TestRequest {
    TestRequest::get()
        .uri("/whoami")
}

#[allow(dead_code)]
pub fn ticket_new(shop: &str, departments: &[&str], est_minutes: i32) -> TestRequest {
    TestRequest::post()
        .uri(&format!("/shop/{shop_id}/ticket/new", shop_id=shop))
        .set_json(&TicketNewRequest {
            department_ids: departments.iter().map(|&s|String::from(s)).collect(),
            est_minutes,
        })
} 

#[allow(dead_code)]
pub fn ticket_est(uid: &str) -> TestRequest {
    TestRequest::get()
        .uri(&format!("/ticket/est?uid={uid}", uid=uid))
}

#[allow(dead_code)]
pub fn tokens() -> TestRequest {
    TestRequest::get()
        .uri(&"/tokens")
}

#[allow(dead_code)]
pub fn shop_queue(shop_id: &str) -> TestRequest {
    TestRequest::get()
        .uri(&format!("/shop/{shop_id}/ticket/queue", shop_id=shop_id))
}