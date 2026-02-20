mod user_routes;
mod auth_routes;

use axum::Router;
use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api/user", user_routes::router())
        .nest("/api/auth", auth_routes::router())

}
