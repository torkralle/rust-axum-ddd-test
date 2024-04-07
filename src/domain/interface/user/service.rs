use crate::{
    entities::user::user::{self, Entity as User},
    services::user::user::CreateUserInput,
};
use anyhow::{Error, Result};
use sea_orm::DbErr;

#[derive(Debug)]
pub struct FetchUsersOutput {
    pub users: Vec<user::Model>,
}

pub trait UserServiceInterface {
    async fn get_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn get_users(&self) -> Result<FetchUsersOutput, Error>;
    async fn create_user(
        &self,
        create_user_input: CreateUserInput,
    ) -> Result<user::ActiveModel, DbErr>;
}
