use crate::domain::aggregate::{user::User, value_object::user_id::UserId};
use crate::services::create_user::{CreateUserInput, CreateUserOutput, CreateUserService};
use crate::services::fetch_users::{FetchUsersOutput, FetchUsersUsecase};
use crate::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct CreateUserRequestBody {
    pub name: String,
    pub email: String,
}

impl std::convert::From<CreateUserRequestBody> for CreateUserInput {
    fn from(CreateUserRequestBody { name, email }: CreateUserRequestBody) -> Self {
        CreateUserInput::new(name, email)
    }
}

#[derive(Debug, Deserialize, Serialize)]

pub struct CreateUserResponseBody {
    pub id: usize,
    pub name: String,
    pub email: String,
}
impl std::convert::From<CreateUserOutput> for CreateUserResponseBody {
    fn from(CreateUserOutput { id, name, email }: CreateUserOutput) -> Self {
        CreateUserResponseBody { id, name, email }
    }
}

// Handler for post /users
pub async fn handle_create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> impl IntoResponse {
    let create_user_input = CreateUserInput::from(body);
    let mut service = CreateUserService::new(state.user_repository);
    service
        .execute(create_user_input)
        .map(CreateUserResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

// Handler for get /users
// #[axum_macros::debug_handler]
// pub async fn handle_get_users() -> Json<Vec<User>> {
//     let user1 = User {
//         id: UserId::gen(),
//         name: "Hoshiko".to_string(),
//         email: "test@gmail..com".to_string(),
//     };
//     let user2 = User {
//         id: UserId::gen(),
//         name: "John".to_string(),
//         email: "john@doe.com".to_string(),
//     };
//     let users = vec![user1, user2];
//     Json(users)
// }

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetchUsersResponseBody {
    pub users: Vec<User>,
}

impl std::convert::From<FetchUsersOutput> for FetchUsersResponseBody {
    fn from(FetchUsersOutput { users }: FetchUsersOutput) -> Self {
        FetchUsersResponseBody { users }
    }
}

pub async fn handle_get_users(
    State(state): State<AppState>,
    // Path(param): Path<FetchUsersInputParam>,
) -> Result<Json<FetchUsersResponseBody>, String> {
    // let fetch_circle_input = FetchUsersInput::new(param.id);
    let usecase = FetchUsersUsecase::new(state.user_repository);
    usecase
        .execute()
        .map(FetchUsersResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
