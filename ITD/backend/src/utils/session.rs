use actix_session::Session;
use serde::{Serialize, Deserialize};

const KEY_CUSTOMER_ACCOUNT: &'static str = "customer_account";
const KEY_STAFF_ACCOUNT: &'static str = "staff_account";

#[derive(Serialize, Deserialize)]
pub struct StaffAccount {
    id: i32,
    shop_id: i32,
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

pub fn get_staff_account(session: &Session) -> Option<StaffAccount> {
    session.get::<Option<StaffAccount>>(KEY_STAFF_ACCOUNT).unwrap()
        .and_then(|o| o)
}

pub fn set_staff_account(session: &Session, id: i32, shop_id: i32) {
    session.set(KEY_STAFF_ACCOUNT, Some(StaffAccount{id, shop_id})).unwrap();
}

pub fn clear_staff_account(session: &Session) {
    session.set::<Option<StaffAccount>>(KEY_STAFF_ACCOUNT, None).unwrap();
}