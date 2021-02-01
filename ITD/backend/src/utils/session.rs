use actix_session::Session;
use serde::{Serialize, Deserialize};

use super::encoding::decode_serial;

const KEY_CUSTOMER_ACCOUNT: &'static str = "customer_account";
const KEY_STAFF_ACCOUNT: &'static str = "staff_account";

#[derive(Serialize, Deserialize)]
pub struct StaffSession {
    pub id: i32,
    pub shop_id: i32,
}

pub fn get_account(session: &Session) -> Option<i32> {
    session.get::<Option<i32>>(KEY_CUSTOMER_ACCOUNT).unwrap()
        .and_then(|o| o)
}

pub fn set_account(session: &Session, uid: i32) {
    session.set(KEY_CUSTOMER_ACCOUNT, Some(uid)).unwrap();
}

pub fn clear_account(session: &Session) {
    session.set::<Option<i32>>(KEY_CUSTOMER_ACCOUNT, None).unwrap();
}

pub fn get_staff_account(session: &Session) -> Option<StaffSession> {
    session.get::<Option<StaffSession>>(KEY_STAFF_ACCOUNT).unwrap()
        .and_then(|o| o)
}

pub fn set_staff_account(session: &Session, id: i32, shop_id: i32) {
    session.set(KEY_STAFF_ACCOUNT, Some(StaffSession{id, shop_id})).unwrap();
}

pub fn clear_staff_account(session: &Session) {
    session.set::<Option<StaffSession>>(KEY_STAFF_ACCOUNT, None).unwrap();
}

pub fn check_staff_auth(session: &Session, shop_id: &str) -> Option<StaffSession> {
    let shop_id = decode_serial(shop_id).ok()?;
    let staff = get_staff_account(&session)?;
    if shop_id != staff.shop_id {
        Some(staff)
    } else {
        None
    }
}