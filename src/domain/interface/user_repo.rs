use crate::domain::aggregate::user::{self, User};
use crate::domain::aggregate::value_object::user_id::UserId;
use anyhow::{Error, Result};

pub trait UserRepositoryInterface {
    // fn find_user_by_id(&self, user_id: &UserId) -> Result<User, Error>;
    fn read_users(&self) -> Result<Vec<User>, Error>;
    fn create(&self, user: &User) -> Result<(), Error>;
    // fn update(&self, user: &User) -> Result<User, Error>;
    // fn delete(&self, user: &User) -> Result<(), Error>;
}
