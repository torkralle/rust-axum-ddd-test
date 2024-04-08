use crate::domain::user::dto::{
    CreateUserDTO, CreateUserRequestBody, UpdateUserDTO, UpdateUserRequestBody,
};
use crate::domain::user::model as user;
use crate::domain::user::result::FetchUsersResponseBody;
use crate::domain::user::service::UserServiceInterface;
use crate::services::user::user::UserService;
use crate::AppState;
use axum::{extract::Path, Json};
use sea_orm::TryIntoModel;
use std::sync::Arc;

// todo: それぞれでServiceを作成しているので、一つにまとめたい。
// 一度やってみたがだめだった。実装例探す
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
    let dto = CreateUserDTO::from(body);
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    match service.create_user(dto).await {
        Ok(r) => Ok(Json(r)),
        Err(_) => panic!("db error"),
    }
}

pub async fn handle_update_user(
    state: Arc<AppState>,
    Json(body): Json<UpdateUserRequestBody>,
) -> Result<Json<user::Model>, String> {
    let dto = UpdateUserDTO::from(body);
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    match service.update_user(dto).await {
        Ok(r) => Ok(Json(r)),
        Err(_) => panic!("db error"),
    }
}

pub async fn handle_delete_user_by_id(
    state: Arc<AppState>,
    Path(id): Path<String>,
) -> Result<Json<String>, String> {
    let parsed_id: i32 = id.parse().unwrap();
    let ss = (*state).clone();
    let service = UserService::new(ss.user_repository);
    match service.delete_user(parsed_id).await {
        Ok(_) => Ok(Json("OK".to_string())),
        Err(_) => panic!("db error"),
    }
}
