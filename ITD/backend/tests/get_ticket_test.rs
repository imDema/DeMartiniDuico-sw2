
mod common;
use clup::api::ticket::TokensResponse;
use clup::models::ticket::TicketResponse;
use clup::setup_db;
use clup::utils::encoding::encode_serial;
use clup::utils::tests::{test_department, test_shop};
use common::requests::*;

use actix_web::http::StatusCode;
use actix_web::test;

#[actix_rt::test]
async fn register_test() -> sqlx::Result<()> {
    let mut app = setup_app!();

    let (email, _, session) = quick_create_customer!(&mut app);

    let r = req!(whoami(), &session, &mut app);
    let r_body = read_utf8_body(r).await;
    assert!(r_body.contains(&email));

    let (s0, d00, d01, s1, d10) = async { // TODO: use endpoints to create shop
        let conn = setup_db(&std::env::var("DATABASE_URL").unwrap()).await;
        let sid = test_shop(&conn).await.unwrap();
        let s0 = encode_serial(sid);
        let did0 = test_department(&conn, sid).await.unwrap();
        let did1 = test_department(&conn, sid).await.unwrap();
        let d00 = encode_serial(did0);
        let d01 = encode_serial(did1);
        let sid = test_shop(&conn).await.unwrap();
        let s1 = encode_serial(sid);
        let did = test_department(&conn, sid).await.unwrap();
        let d10 = encode_serial(did);
        (s0, d00, d01, s1, d10)
    }.await;

    let r = req!(ticket_new(&s0, &[&d01], 15), &mut app); //No session should 401
    assert_eq!(r.status(), StatusCode::FORBIDDEN);
    
        let r = req!(tokens(), &mut app); //No session should 401
        assert_eq!(r.status(), StatusCode::FORBIDDEN);

    // First ticket

    let ticket = ticket!(&s0, [&d00, &d01], 15, &session, &mut app);

    check_tokens!([&ticket], &session, &mut app);

    // Second ticket

    let ticket_2 = ticket!(&s1, [&d10], 20, &session, &mut app);

    check_tokens!([&ticket, &ticket_2], &session, &mut app);

    Ok(())
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

#[macro_export]
macro_rules! check_tokens {
    ([$($ticket:expr),+],$sess:expr, $app:expr) => {
        let r = req!(tokens(), $sess, $app);
        assert_eq!(r.status(), StatusCode::OK);

        let toks: TokensResponse = test::read_body_json(r).await;

        assert_eq!(toks.bookings.len(), 0);
        $(
        assert!(toks.tickets.contains($ticket));
        )+
    };
}