use serde::Deserialize;

use crate::domain::aggregate::user::User;
use crate::domain::aggregate::value_object::user_id;
use crate::domain::interface::user_repo::UserRepositoryInterface;
use anyhow::{Error, Result};

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
    pub user_id: usize,
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

    pub fn execute(
        &mut self,
        create_user_input: CreateUserInput,
    ) -> Result<CreateUserOutput, Error> {
        // todo: 最後に?でエラー処理したい
        let user = User::new(create_user_input.name, create_user_input.email);
        self.user_repository
            .create(&user)
            .map(|_| CreateUserOutput {
                user_id: usize::from(user.id),
                name: user.name,
                email: user.email,
            })
    }
}
