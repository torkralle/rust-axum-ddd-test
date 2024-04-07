use crate::domain::interface::user::service::{FetchUsersOutput, UserServiceInterface};
use crate::entities::user;
use crate::services::user::user::{CreateUserInput, CreateUserOutput, UserService};
use crate::AppState;
use axum::body::Body;
use axum::{
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use sea_orm::TryIntoModel;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
) -> Result<Json<user::Model>, String> {
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);

    let parsed_id: i32 = id.parse().unwrap();
    println!("{}", parsed_id);
    match service.get_user_by_id(parsed_id).await {
        Ok(r) => match r {
            Some(u) => Ok(Json(u)),
            None => Err("no record".to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn handle_create_user(
    state: Arc<AppState>,
    Json(body): Json<CreateUserRequestBody>,
) -> Result<Json<user::Model>, String> {
    let create_user_input = CreateUserInput::from(body);
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    match service.create_user(create_user_input).await {
        Ok(r) => match r.try_into_model() {
            Ok(m) => Ok(Json(m)),
            Err(_) => panic!("model error"),
        },
        Err(_) => panic!("db error"),
    }
}
