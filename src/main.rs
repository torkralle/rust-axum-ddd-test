use anyhow::Error;
use handler::router;
use infra::user::user_repo::UserRepository;
use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc};
use tracing_subscriber;
mod controllers;
mod domain;
mod handler;
mod infra;
mod services;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    user_repository: UserRepository,
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
