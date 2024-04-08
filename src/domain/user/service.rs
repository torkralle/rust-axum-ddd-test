use super::dto::{CreateUserDTO, UpdateUserDTO};
use crate::domain::user::model as user;
use anyhow::{Error, Result};
use sea_orm::{DbErr, DeleteResult};

#[derive(Debug)]
pub struct FetchUsersOutput {
    pub users: Vec<user::Model>,
}

pub trait UserServiceInterface {
    async fn get_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn get_users(&self) -> Result<FetchUsersOutput, Error>;
    async fn create_user(&self, dto: CreateUserDTO) -> Result<user::Model, DbErr>;
    async fn update_user(&self, dto: UpdateUserDTO) -> Result<user::Model, DbErr>;
    async fn delete_user(&self, id: i32) -> Result<DeleteResult, DbErr>;
}
