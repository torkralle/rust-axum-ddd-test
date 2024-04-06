use std::env;

use anyhow::Error;
use axum::{
    routing::{get, post},
    Router,
};
use futures::executor::block_on;
use infra::user_repo::UserRepository;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use thiserror::Error;
use tracing::{info, info_span};
use tracing_subscriber;
mod domain;
mod entities;
mod handler;
mod infra;
mod services;
use crate::handler::{handle_create_user, handle_get_users};
use dotenv::dotenv;

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

async fn new_db() -> Result<DatabaseConnection, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("should be fetched url");
    let db = Database::connect(database_url).await?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let db;
    match new_db().await {
        Ok(created_db) => db = created_db,
        Err(e) => panic!("{}", e),
    }
    let state = AppState {
        user_repository: UserRepository::new(db),
    };
    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
