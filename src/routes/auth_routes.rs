use axum::{Router, routing::post};
use crate::{AppState, handlers::auth_handler, middleware::auth};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth_handler::login))
        .route("/register", post(auth_handler::register))
}

pub fn protected_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", post(auth_handler::logout))
        .route("/change-password", post(auth_handler::change_password))
        .route_layer(axum::middleware::from_fn_with_state(state, auth))
}