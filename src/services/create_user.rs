use sea_orm::{DbErr, Set, TryIntoModel};
use serde::Deserialize;

use crate::domain::interface::user_repo::UserRepositoryInterface;
use crate::entities::user;
use anyhow::Result;

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

pub struct CreateUserService<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T> CreateUserService<T>
where
    T: UserRepositoryInterface,
{
    pub fn new(user_repository: T) -> Self {
        CreateUserService { user_repository }
    }

    pub async fn execute(
        &mut self,
        create_user_input: CreateUserInput,
    ) -> Result<user::Model, DbErr> {
        let user = user::ActiveModel {
            name: Set(create_user_input.name.to_owned()),
            email: Set(create_user_input.email.to_owned()),
            ..Default::default()
        };
        let result = self.user_repository.create(user).await?;
        Ok(result.try_into_model()?)
    }
}
