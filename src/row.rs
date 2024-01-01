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
