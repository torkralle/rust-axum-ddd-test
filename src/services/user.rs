use sea_orm::{DbErr, Set, TryIntoModel};
use serde::Deserialize;

use crate::domain::interface::user::repository::UserRepositoryInterface;
use crate::domain::interface::user::service::{FetchUsersOutput, UserServiceInterface};
use crate::entities::user;
use anyhow::{Error, Result};

// todo: Userサービスにまとめたいところ
#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

impl CreateUserInput {
    pub fn new(name: String, email: String) -> Self {
        CreateUserInput { name, email }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserOutput {
    // pub id: usize,
    pub name: String,
    pub email: String,
}

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
    async fn create_user(
        &self,
        create_user_input: CreateUserInput,
    ) -> Result<user::ActiveModel, DbErr> {
        let user = user::ActiveModel {
            name: Set(create_user_input.name.to_owned()),
            email: Set(create_user_input.email.to_owned()),
            ..Default::default()
        };
        let result = self.user_repository.create_user(user).await?;
        Ok(result.try_into_model()?.into())
    }

    async fn get_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        todo!()
    }

    async fn get_users(&self) -> Result<FetchUsersOutput, Error> {
        let result = self.user_repository.read_users().await;
        result.map(|users: Vec<user::Model>| FetchUsersOutput { users: users })
    }
}
