use crate::domain::aggregate::value_object::user_id::UserId;

use super::value_object::{grade::Grade, major::Major};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: UserId, // メンバーのID (Value Object)
    pub name: String,
    pub email: String,
}

impl User {
    // ユーザー新規作成
    pub fn new(name: String, age: usize, grade: Grade, major: Major) -> Self {
        User {
            id: UserId::gen(),
            name,
            email,
        }
    }

    // メンバーの再構成メソッド
    pub fn reconstruct(id: UserId, name: String, age: usize, grade: Grade, major: Major) -> Self {
        User { id, name, age }
    }
}
