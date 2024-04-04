use crate::domain::{
    self,
    aggregate::{user::User, value_object::user_id::UserId},
};
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Error, Json,
};

use serde::{Deserialize, Serialize};

// Handler for /create-user
pub async fn handle_create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

// Handler for /users
pub async fn handle_get_users() -> Result<Json<Vec<User>>, Error> {
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
    Ok(Json(users))
}
