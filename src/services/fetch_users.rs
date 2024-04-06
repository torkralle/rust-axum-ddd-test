use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::domain::interface::user_repo::UserRepositoryInterface;

use crate::entities::user;

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
    pub users: Vec<user::Model>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberOutput {
    pub id: usize,
    pub name: String,
    pub age: usize,
    pub grade: usize,
    pub major: String,
}
pub struct FetchUsersService<T>
where
    T: UserRepositoryInterface,
{
    user_repository: T,
}

impl<T> FetchUsersService<T>
where
    T: UserRepositoryInterface,
{
    pub fn new(user_repository: T) -> Self {
        FetchUsersService { user_repository }
    }

    pub async fn execute(
        &self,
        // fetch_user_input: FetchUserInput,
    ) -> Result<FetchUsersOutput, Error> {
        let result = self.user_repository.read_users().await;
        result.map(|users: Vec<user::Model>| FetchUsersOutput { users: users })
    }
}
