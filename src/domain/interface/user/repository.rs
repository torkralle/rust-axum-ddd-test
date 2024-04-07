use crate::entities::{user, user::Entity as User};
use anyhow::{Error, Result};
use sea_orm::DbErr;

pub trait UserRepositoryInterface {
    async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn read_users(&self) -> Result<Vec<user::Model>, Error>;
    async fn create_user(&self, user: user::ActiveModel) -> Result<user::ActiveModel, DbErr>;
    // fn update(&self, user: &User) -> Result<User, Error>;
    // fn delete(&self, user: &User) -> Result<(), Error>;
}
