use crate::AppState;
use crate::handlers::user_handler;
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_handler::get_me))
        .route("/all", get(user_handler::list_users))
        .route("/{id}", get(user_handler::get_user))
        .route("/{id}/update", patch(user_handler::update_user))
        .route("/{id}/delete", post(user_handler::delete_user))
}
