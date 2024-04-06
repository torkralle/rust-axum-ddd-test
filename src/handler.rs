use crate::entities::user;
use crate::services::create_user::{CreateUserInput, CreateUserOutput, CreateUserService};
use crate::services::fetch_users::{FetchUsersOutput, FetchUsersService};
use crate::AppState;
use axum::body::Body;
use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use sea_orm::{DbErr, TryIntoModel};
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
    pub name: String,
    pub email: String,
}
impl std::convert::From<CreateUserOutput> for CreateUserResponseBody {
    fn from(CreateUserOutput { name, email }: CreateUserOutput) -> Self {
        CreateUserResponseBody { name, email }
    }
}

// Handler for post /users
pub async fn handle_create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> impl IntoResponse {
    let create_user_input = CreateUserInput::from(body);
    let mut service = CreateUserService::new(state.user_repository);
    let result = service.execute(create_user_input).await;
    // let response;
    // match result {
    // Ok(r)=>    r.try_into_model().map_err(Into::into),
    // Err(e) => Err(e.into())
    // }
    // result.map(|r| r.try_into_model()).map_err(Into::into)
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
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
    pub users: Vec<user::Model>,
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
    let service = FetchUsersService::new(state.user_repository);
    match service.execute().await {
        Ok(r) => Ok(Json(r.into())),
        Err(e) => Err(e.to_string()),
    }
}
