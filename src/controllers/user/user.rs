use std::sync::Arc;

use crate::domain::interface::user::service::{FetchUsersOutput, UserServiceInterface};
use crate::entities::user;
// use crate::services::fetch_users::{FetchUsersOutput, FetchUsersService};
use crate::services::user::user::{CreateUserInput, CreateUserOutput, UserService};
use crate::AppState;
use axum::body::Body;
use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use futures::future::Shared;
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetchUsersResponseBody {
    pub users: Vec<user::Model>,
}

impl std::convert::From<FetchUsersOutput> for FetchUsersResponseBody {
    fn from(FetchUsersOutput { users }: FetchUsersOutput) -> Self {
        FetchUsersResponseBody { users }
    }
}

#[derive(Clone)]
pub struct UserController<T>
where
    T: UserServiceInterface,
{
    user_service: T,
}

impl<T: UserServiceInterface> UserController<T> {
    pub fn new(user_service: T) -> Self {
        UserController { user_service }
    }
}

// pub struct UserController<T>
// where
//     T: Clone, // TがCloneトレイトを実装している必要がある
// {

// }

// Handler for post /users
pub async fn handle_create_user(
    state: Arc<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> impl IntoResponse {
    let create_user_input = CreateUserInput::from(body);
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    // let result = .user_service.create_user(create_user_input).await;
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

pub async fn handle_get_users(
    state: Arc<AppState>,
    // Path(param): Path<FetchUsersInputParam>,
) -> Result<Json<FetchUsersResponseBody>, String> {
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    match service.get_users().await {
        Ok(r) => Ok(Json(r.into())),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn handle_get_user_by_id(
    Path(id): Path<String>,
    state: Arc<AppState>,
) -> Result<String, String> {
    println!("{}", id);
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    match service.get_users().await {
        Ok(r) => Ok(id),
        Err(e) => Err(e.to_string()),
    }
}
