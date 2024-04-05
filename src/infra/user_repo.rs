use super::db::Db;
use crate::domain::{
    aggregate::{user::User, value_object::user_id::UserId},
    interface::user_repo::UserRepositoryInterface,
};
use anyhow::{Error, Result};

#[derive(Clone, Debug)]
pub struct UserRepository {
    db: Db,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct UserData {
    id: usize,
    name: String,
    email: String,
}

impl UserRepository {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

impl UserRepositoryInterface for UserRepository {
    fn create(&self, user: &User) -> Result<(), Error> {
        match self.db.get::<UserData, _>(&user.id.to_string())? {
            Some(_) => Err(Error::msg("User already exists")),
            None => {
                self.db
                    .set(user.id.to_string(), &UserData::from(user.clone()))?;
                Ok(())
            }
        }
    }
}

impl std::convert::From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            id: user.id.into(),
            name: user.name,
            email: user.email,
        }
    }
}
