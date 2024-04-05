use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::domain::{
    aggregate::{user::User, value_object::user_id::UserId},
    interface::user_repo::UserRepositoryInterface,
};

// // #[derive(Debug, Deserialize)]
// pub struct FetchUserInput {
//     pub id: usize,
// }

// impl FetchUserInput {
//     pub fn new(id: usize) -> Self {
//         FetchUserInput { id }
//     }
// }

#[derive(Debug)]
pub struct FetchUsersOutput {
    pub users: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberOutput {
    pub id: usize,
    pub name: String,
    pub age: usize,
    pub grade: usize,
    pub major: String,
}
pub struct FetchUsersUsecase<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T> FetchUsersUsecase<T>
where
    T: UserRepositoryInterface,
{
    pub fn new(user_repository: T) -> Self {
        FetchUsersUsecase { user_repository }
    }

    pub fn execute(
        &self,
        // fetch_user_input: FetchUserInput,
    ) -> Result<FetchUsersOutput, Error> {
        // let user_id = UserId::from(fetch_user_input.id);
        self.user_repository
            .read_users()
            .map(|users: Vec<User>| FetchUsersOutput { users: users })
    }
}
