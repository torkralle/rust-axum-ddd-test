use std::env;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Error, Json, Router,
};
use serde::{Deserialize, Serialize};

// type定義
// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

// Handler for /create-user
pub async fn handle_create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

// Handler for /users
pub async fn handle_get_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}
