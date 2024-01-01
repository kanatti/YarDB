use std::fmt;

use crate::constants::{EMAIL_SIZE, USER_NAME_SIZE};

/// Hard Coded Schema: id(int), username(string), email(string)
///
/// Example: `insert 1 foo foo@bar.com`
#[derive(Debug)]
pub struct Row {
    pub id: i32,
    pub username: Box<[u8; USER_NAME_SIZE]>,
    pub email: Box<[u8; EMAIL_SIZE]>,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let username = String::from_utf8_lossy(self.username.as_slice());
        let email = String::from_utf8_lossy(self.email.as_slice());
        write!(f, "{} {} {}", self.id, username, email)
    }
}
