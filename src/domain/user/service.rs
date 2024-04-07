use super::dto::{CreateUserDTO, UpdateUserDTO};
use crate::domain::user::model as user;
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
        create_user_input: CreateUserDTO,
    ) -> Result<user::ActiveModel, DbErr>;
    async fn update_user(&self, update_user_input: UpdateUserDTO) -> Result<user::Model, DbErr>;
}
