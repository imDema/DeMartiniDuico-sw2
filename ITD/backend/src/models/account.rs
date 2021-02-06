use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use rand::Rng;

/// Internal Account structure, contains login related information and methods for checking authentication
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Account {
    pub(super) id: i32,
    pub(super) email: String,
    pub(super) salt: Vec<u8>,
    pub(super) digest: Vec<u8>,
}

/// Salt and digest produced by the password hashing function
pub struct HashedPass {
    pub salt: Vec<u8>,
    pub digest: Vec<u8>,
}

impl Account {
    fn argon_conf<'a>() -> argon2::Config<'a> { argon2::Config::default() }

    /// Check if `password` is correct for this accont
    pub fn verify_authentication(&self, password: &[u8]) -> bool {
        let conf = Self::argon_conf();
        let matches = argon2::verify_raw(password, &self.salt, &self.digest, &conf);
        matches.unwrap_or_else(|err| {log::error!("Argon2 error!: `{:?}`", err); false})
    }

    /// Calculate hash digest for `password`
    pub fn hash_password(password: &[u8]) -> HashedPass {
        let config = Self::argon_conf();
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill(&mut salt[..]);
        let digest = argon2::hash_raw(password, &salt, &config).expect("Argon2 error!");
        HashedPass{salt, digest}
    }

    pub fn id(&self) -> i32 {self.id}
    pub fn email(&self) -> &str {&self.email}
    pub fn salt(&self) -> &[u8] {&self.salt[..]}
    pub fn digest(&self) -> &[u8] {&self.digest[..]}
}


#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn authentication_test() {
        let pass = "Please use a long password!".as_bytes();
        let p = Account::hash_password(&pass);

        let cust = Account{
            id: 123,
            email: "123@mail.com".to_owned(),
            salt: p.salt,
            digest: p.digest,
        };
        
        assert!(cust.verify_authentication(&pass));
        assert!(!cust.verify_authentication(&"Another password!".as_bytes()));
        assert!(!cust.verify_authentication(&"please use a long password!".as_bytes()));
        assert!(!cust.verify_authentication(&"Please use a long password".as_bytes()));
    }

    #[test]
    fn distinct_salt_test() {
        let pass = "Please use a long password!".as_bytes();
        let p1 = Account::hash_password(&pass);
        let p2 = Account::hash_password(&pass);

        if p1.salt == p2.salt {
            eprintln!("Got the same salt!");
            assert_eq!(p1.digest, p2.digest);
            distinct_salt_test(); // Since we got the same salt we want to test again
        } else {
            assert_ne!(p1.digest, p2.digest);
        }
    }
}