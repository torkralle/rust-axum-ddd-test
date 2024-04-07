use super::{model as user, service::FetchUsersOutput};
use serde::{Deserialize, Serialize};

// TODO: 整理する
#[derive(Debug, Deserialize)]
pub struct CreateUserOutput {
    // pub id: usize,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct CreateUserResponseBody {
    pub name: String,
    pub email: String,
}

impl std::convert::From<CreateUserOutput> for CreateUserResponseBody {
    fn from(CreateUserOutput { name, email }: CreateUserOutput) -> Self {
        CreateUserResponseBody { name, email }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetchUsersResponseBody {
    pub users: Vec<user::Model>,
}

impl std::convert::From<FetchUsersOutput> for FetchUsersResponseBody {
    fn from(FetchUsersOutput { users }: FetchUsersOutput) -> Self {
        FetchUsersResponseBody { users }
    }
}

/// The result of a DELETE operation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeleteResult {
    /// The number of rows affected by the DELETE operation
    pub rows_affected: u64,
}
