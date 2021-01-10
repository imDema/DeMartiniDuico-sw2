use actix_session::Session;

const KEY_ACCOUNT: &'static str = "account";

pub fn get_account(session: &Session) -> Option<i32> {
    session.get::<Option<i32>>(KEY_ACCOUNT).unwrap()
        .and_then(|o| o)
}

pub fn set_account(session: &Session, uid: i32) {
    session.set(KEY_ACCOUNT, Some(uid)).unwrap();
}

pub fn clear_account(session: &Session) {
    session.set::<Option<i32>>(KEY_ACCOUNT, None).unwrap();
}