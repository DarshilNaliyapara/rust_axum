use axum::{Router, routing::{get, post}};
use crate::AppState;
use crate::handlers::auth_handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(auth_handler::login))
        .route("/register", post(auth_handler::register))
        .route("/logout", get(auth_handler::logout))
}
