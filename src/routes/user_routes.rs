use crate::AppState;
use crate::handlers::user_handler;
use crate::middleware::{auth, is_admin};
use axum::middleware::from_fn;
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(user_handler::get_me))
        .route(
            "/all",
            get(user_handler::list_users).route_layer(from_fn(is_admin)),
        )
        .route("/{id}", get(user_handler::get_user))
        .route("/{id}/update", patch(user_handler::update_user))
        .route("/{id}/delete", post(user_handler::delete_user))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth,
        ))
}
