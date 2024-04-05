use axum::{
    routing::{get, post},
    Router,
};
use infra::user_repo::UserRepository;
use sea_orm::{Database, DatabaseConnection};
use thiserror::Error;
use tracing::{info, info_span};
use tracing_subscriber;
mod domain;
mod handler;
mod infra;
mod services;
use crate::handler::{handle_create_user, handle_get_users};

#[derive(Clone)]
struct AppState {
    user_repository: UserRepository,
}

fn router() -> Router<AppState> {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(|| async { "Hello world!" }))
        // `POST /users` goes to `create_user`
        .route("/users", get(handle_get_users).post(handle_create_user))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let state = AppState {
        user_repository: UserRepository::new(),
    };
    let app = router().with_state(state);

    // build our application with a route

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Error, Debug)]
enum MyError {
    // #[error("Failed to get connection")]
    // ConnectionPoolError(#[from] r2d2::Error),
    #[error("Failed SQL execution")]
    SQLiteError(#[from] sea_orm::RuntimeErr),
}
