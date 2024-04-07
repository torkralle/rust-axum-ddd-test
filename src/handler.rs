use crate::domain::interface::user::service::{FetchUsersOutput, UserServiceInterface};
use crate::entities::user;
// use crate::services::fetch_users::{FetchUsersOutput, FetchUsersService};
use crate::services::user::{CreateUserInput, CreateUserOutput, UserService};
use crate::AppState;
use axum::body::Body;
use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};

// todo: それぞれでServiceを作成しているので、一つにまとめる

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
    let mut service = UserService::new(state.user_repository);
    let result = service.create_user(create_user_input).await;
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
    let service = UserService::new(state.user_repository);
    match service.get_users().await {
        Ok(r) => Ok(Json(r.into())),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn handle_get_user_by_id(
    Path(id): Path<String>,
    state: AppState,
) -> Result<String, String> {
    println!("{}", id);
    let service = UserService::new(state.user_repository);
    match service.get_users().await {
        Ok(r) => Ok(id),
        Err(e) => Err(e.to_string()),
    }
}
