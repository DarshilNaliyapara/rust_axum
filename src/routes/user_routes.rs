use axum::{Router, routing::{get, post}};
use crate::AppState;
use crate::handlers::user_handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_handler::get_me))
        .route("/{id}", get(user_handler::get_user))
        .route("/all", get(user_handler::list_users))
        .route("/update", get(user_handler::update_user))
        .route("/{id}/delete", post(user_handler::delete_user))
}
