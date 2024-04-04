use crate::domain::aggregate::value_object::user_id::UserId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: UserId, // メンバーのID (Value Object)
    pub name: String,
    pub email: String,
}

impl User {
    // ユーザー新規作成
    pub fn new(name: String, email: String) -> Self {
        User {
            id: UserId::gen(),
            name,
            email,
        }
    }
}
