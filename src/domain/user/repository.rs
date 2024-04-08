use super::query::{CreateUserQuery, UpdateUserQuery};
use crate::domain::user::model as user;
use anyhow::{Error, Result};
use sea_orm::{DbErr, DeleteResult};

pub trait UserRepositoryInterface {
    async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn read_users(&self) -> Result<Vec<user::Model>, Error>;
    async fn create_user(&self, query: CreateUserQuery) -> Result<user::ActiveModel, DbErr>;
    async fn update_user(&self, query: UpdateUserQuery) -> Result<user::Model, DbErr>;
    async fn delete_user(&self, id: i32) -> Result<DeleteResult, DbErr>;
}
