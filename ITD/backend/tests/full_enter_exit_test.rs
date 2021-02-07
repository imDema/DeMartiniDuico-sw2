
mod common;
use clup::api::ticket::TicketEstResponse;
use clup::models::ticket::TicketResponse;
use clup::setup_db;
use clup::utils::encoding::encode_serial;
use clup::utils::tests::{test_department, test_shop};
use common::requests::*;

use actix_web::http::StatusCode;
use actix_web::test;

#[actix_rt::test]
async fn full_enter_exit_test() -> sqlx::Result<()> {
    pretty_env_logger::init();
    let mut app = setup_app!();

    let (s0, d0, d1) = async {
        let conn = setup_db(&std::env::var("DATABASE_URL").unwrap()).await;
        let sid = test_shop(&conn).await.unwrap();
        let s0 = encode_serial(sid);
        let did0 = test_department(&conn, sid, 2).await.unwrap();
        let did1 = test_department(&conn, sid, 5).await.unwrap();
        let d0 = encode_serial(did0);
        let d1 = encode_serial(did1);
        (s0, d0, d1)
    }.await;

    let (_, _, customer_0) = quick_create_customer!(&mut app);
    let (_, _, customer_1) = quick_create_customer!(&mut app);
    let (_, _, customer_2) = quick_create_customer!(&mut app);

    let (_, _, staff) = quick_create_staff!(&mut app, &s0);

    let t0 = ticket!(&s0, [&d0, &d1], 15, &customer_0, &mut app);
    let t1 = ticket!(&s0, [&d0], 15, &customer_1, &mut app);
    let t2 = ticket!(&s0, [&d0, &d1], 15, &customer_2, &mut app);

    let (t0, t1, t2) = (t0.uid, t1.uid, t2.uid);

    let r = req!(ticket_est(&t0), &customer_1, &mut app); // C1 tries to use another C0's ticket
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);

    let r = req!(ticket_est(&t0), &customer_0, &mut app); // C0 checks queue
    assert_eq!(r.status(), StatusCode::OK);
    let resp: TicketEstResponse = test::read_body_json(r).await;
    assert_eq!(resp.people, 0);

    let r = req!(log_entry(&s0, &t0), &staff, &mut app); // C0 enters
    assert_eq!(r.status(), StatusCode::OK);

    let r = req!(log_entry(&s0, &t0), &staff, &mut app); // C0 cannot enter twice
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);

    let r = req!(log_entry(&s0, &t2), &staff, &mut app); // C2 can't enter, not first in line
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);

    let r = req!(ticket_est(&t2), &customer_2, &mut app); // C2 checks queue
    assert_eq!(r.status(), StatusCode::OK);
    let resp: TicketEstResponse = test::read_body_json(r).await;
    assert_eq!(resp.people, 1);
    eprintln!("{:?}", resp);

    let r = req!(log_entry(&s0, &t1), &staff, &mut app); // C1 enters
    assert_eq!(r.status(), StatusCode::OK);

    let r = req!(log_entry(&s0, &t2), &staff, &mut app); // C2 can't enter, department is full
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);

    let r = req!(log_exit(&s0, &t0), &staff, &mut app); // C0 exits
    assert_eq!(r.status(), StatusCode::OK);

    let r = req!(log_entry(&s0, &t2), &staff, &mut app); // C2 can now enter
    assert_eq!(r.status(), StatusCode::OK);

    let r = req!(log_exit(&s0, &t1), &staff, &mut app); // C1 exits
    assert_eq!(r.status(), StatusCode::OK);

    let r = req!(log_exit(&s0, &t2), &staff, &mut app); // C2 exits
    assert_eq!(r.status(), StatusCode::OK);

    let r = req!(log_exit(&s0, &t2), &staff, &mut app); // C2 cannot exit twice
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);

    let r = req!(log_entry(&s0, &t0), &staff, &mut app); // C1 cannot exit twice
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);

    Ok(())
}


#[macro_export]
macro_rules! log_entry {
    ($tid:expr, $shop:expr, $expect:expr, $sess:expr, $app:expr) => {{
        let r = req!(log_entry($shop,$tid), $sess, $app);
        assert_eq!(r.status(), $expect);

        let t: TicketResponse = test::read_body_json(r).await;
        assert_eq!($shop, &t.shop_id);
        assert_eq!(t.department_ids.len(), dids.len());
        $(
        assert!(t.department_ids.contains($did));
        )+
        assert!(t.valid);
        assert!(t.active);
        t
    }};
}