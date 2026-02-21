use axum::{Router, routing::{post}};
use tower_cookies::CookieManagerLayer;

use crate::{AppState, handlers::auth_handler};


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth_handler::login))
        .route("/register", post(auth_handler::register))
        .route("/logout", post(auth_handler::logout))
        .layer(CookieManagerLayer::new())
}
