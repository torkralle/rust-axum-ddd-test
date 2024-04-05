// use anyhow::Error;

use crate::domain::{
    aggregate::{user::User, value_object::user_id::UserId},
    interface::user_repository_interface::UserRepositoryInterface,
};

use super::db::Db;

#[derive(Clone, Debug)]
pub struct UserRepository {
    db: Db,
}

impl UserRepository {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

impl UserRepositoryInterface for UserRepository {
    fn find_user_by_id(&self, user_id: &UserId) -> Result<User, Error> {
        match self.db.get::<UserData, _>(&user_id.to_string())? {
            Some(data) => Ok(User::try_from(data)?),
            None => Err(Error::msg("User not found")),
        }
    }

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

    fn update(&self, user: &User) -> Result<User, Error> {
        match self.db.get::<UserData, _>(&user.id.to_string())? {
            Some(_) => self
                .db
                .set(user.id.to_string(), &UserData::from(user.clone()))
                .and_then(|_| self.db.get::<UserData, _>(&user.id.to_string()))
                .map(|data| match data {
                    Some(data) => User::try_from(data),
                    None => Err(Error::msg("Failed to convert user data")),
                })?,
            None => Err(Error::msg("User not found")),
        }
    }

    fn delete(&self, user: &User) -> Result<(), Error> {
        match self.db.get::<UserData, _>(&user.id.to_string())? {
            Some(_) => self.db.remove(user.id.to_string()),
            None => Err(Error::msg("User not found")),
        }
    }
}

impl std::convert::From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            id: user.id.into(),
            name: user.name,
        }
    }
}
