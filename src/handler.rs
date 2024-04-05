use crate::domain::{
    self,
    aggregate::{user::User, value_object::user_id::UserId},
};
use axum::{
    body::{self, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
    Error, Json,
};

use serde::{Deserialize, Serialize};

// Handler for post /users
pub async fn handle_create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

// Handler for get /users
#[axum_macros::debug_handler]
pub async fn handle_get_users() -> Json<Vec<User>> {
    let user1 = User {
        id: UserId::gen(),
        name: "Hoshiko".to_string(),
        email: "test@gmail..com".to_string(),
    };
    let user2 = User {
        id: UserId::gen(),
        name: "John".to_string(),
        email: "john@doe.com".to_string(),
    };
    let users = vec![user1, user2];
    Json(users)
}
