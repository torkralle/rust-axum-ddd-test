use crate::domain::user::dto::{CreateUserDTO, GetUsersDTO, UpdateUserDTO};
use crate::domain::user::model as user;
use crate::domain::user::query::{CreateUserQuery, GetUsersQuery, UpdateUserQuery};
use crate::domain::user::repository::UserRepositoryInterface;
use crate::domain::user::service::{FetchUsersOutput, UserServiceInterface};
use anyhow::{Error, Result};
use sea_orm::{DbErr, DeleteResult, TryIntoModel};

#[derive(Clone)]
pub struct UserService<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T: UserRepositoryInterface> UserService<T> {
    pub fn new(user_repository: T) -> Self {
        UserService { user_repository }
    }
}

impl<T: UserRepositoryInterface> UserServiceInterface for UserService<T> {
    async fn get_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        self.user_repository.find_user_by_id(id).await
    }

    async fn get_users(&self, dto: GetUsersDTO) -> Result<FetchUsersOutput, Error> {
        let query = GetUsersQuery::from(dto);
        let result = self.user_repository.read_users(query).await;
        result.map(|users: Vec<user::Model>| FetchUsersOutput { users: users })
    }

    async fn create_user(&self, dto: CreateUserDTO) -> Result<user::Model, DbErr> {
        let query = CreateUserQuery::from(dto);
        let result = self.user_repository.create_user(query).await?;
        Ok(result.try_into_model()?.into())
    }

    async fn update_user(&self, dto: UpdateUserDTO) -> Result<user::Model, DbErr> {
        let query = UpdateUserQuery::from(dto);
        self.user_repository.update_user(query).await
    }

    async fn delete_user(&self, id: i32) -> Result<DeleteResult, DbErr> {
        self.user_repository.delete_user(id).await
    }
}
