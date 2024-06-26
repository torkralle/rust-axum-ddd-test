use crate::{
    controllers::user::user::{
        handle_create_user, handle_delete_user_by_id, handle_get_user_by_id, handle_get_users,
        handle_update_user,
    },
    AppState,
};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router<AppState> {
    let clone_state = Arc::clone(&state);
    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/users",
            get(move |path| handle_get_users(state, path))
                .post({
                    let ss = Arc::clone(&clone_state);
                    move |body| handle_create_user(ss, body)
                })
                .put({
                    let ss = Arc::clone(&clone_state);
                    move |body| handle_update_user(ss, body)
                }),
        )
        .route(
            "/users/:id",
            get({
                let ss = Arc::clone(&clone_state);
                move |query| handle_get_user_by_id(ss, query)
            })
            .delete({
                let ss = Arc::clone(&clone_state);
                move |path| handle_delete_user_by_id(ss, path)
            }),
        )
}
