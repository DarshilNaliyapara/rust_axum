// routes/mod.rs
mod user_routes;
mod auth_routes;

use axum::Router;
use crate::{AppState};

pub fn create_router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .nest("/api/auth", auth_routes::public_router());

    let protected = Router::new()
        .nest("/api/auth", auth_routes::protected_router(state.clone()))
        .nest("/api/user", user_routes::router(state.clone()));

    Router::new()
        .merge(public)
        .merge(protected)
}