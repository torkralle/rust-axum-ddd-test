use std::{borrow::BorrowMut, env, sync::Arc};

use anyhow::Error;
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use controllers::user::user::{handle_create_user, handle_get_user_by_id, handle_get_users};
use infra::user::user_repo::UserRepository;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use services::user::user::UserService;
use tracing::{info, info_span};
use tracing_subscriber;
mod controllers;
mod domain;
mod entities;
mod handler;
mod infra;
mod services;
use dotenv::dotenv;

#[derive(Clone)]
struct AppState {
    user_repository: UserRepository,
}

fn router(state: Arc<AppState>) -> Router<AppState> {
    // let user_service = UserService::new(state.user_repository);
    let clone_state = Arc::clone(&state);
    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/users",
            get(|| handle_get_users(state)).post({
                let ss = Arc::clone(&clone_state);
                move |body| handle_create_user(ss, body)
            }),
        )
        .route(
            "/users/:id",
            get({
                let ss = Arc::clone(&clone_state);
                move |path| handle_get_user_by_id(path, ss)
            }),
        )
}

async fn new_db() -> Result<DatabaseConnection, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("should be fetched url");
    let db = Database::connect(database_url).await?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db;
    match new_db().await {
        Ok(created_db) => db = created_db,
        Err(e) => panic!("{}", e),
    }
    let state = Arc::new(AppState {
        user_repository: UserRepository::new(db),
    });
    let router_state = (*state).clone();
    let app = router(state).with_state(router_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
