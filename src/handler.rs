use crate::domain::aggregate::{user::User, value_object::user_id::UserId};
use crate::services::create_user::CreateUserInput;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
pub struct CreateUserRequestBody {
    pub name: String,
    pub email: String,
}

impl std::convert::From<CreateUserRequestBody> for CreateUserInput {
    fn from(CreateUserRequestBody { name, email }: CreateUserRequestBody) -> Self {
        CreateUserInput::new(name, email)
    }
}

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
