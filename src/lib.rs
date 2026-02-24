use axum::{
    Json,
    extract::{FromRef, FromRequest, FromRequestParts, Request},
    http::{StatusCode, Uri, request::Parts},
    response::{IntoResponse, Response},
};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use serde::de::DeserializeOwned;
use tracing::{error, warn};
use validator::Validate;

pub use config::Pool;
pub use error::AppError;

pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool,
    pub jwt_secret: String,
}

impl FromRef<AppState> for Pool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

pub struct DbConn(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

impl<S> FromRequestParts<S> for DbConn
where
    Pool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        // This decouples the connection's lifetime from the local `pool` variable
        let conn = pool.get_owned().await.map_err(|e| {
            error!(error = %e, "Failed to get owned connection from pool");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
        })?;

        Ok(Self(conn))
    }
}

pub struct ValidJson<T>(pub T);

// Implement FromRequest to consume and validate the body
impl<T, S> FromRequest<S> for ValidJson<T>
where
    // T must be deserializable AND implement our Validate trait
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    // We delegate the actual JSON parsing to Axum's built-in Json extractor
    Json<T>: FromRequest<S>,
{
    // If anything fails, we return a fully formed HTTP Response
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| e.into_response())?; 

        value.validate().map_err(|errors| {
            warn!(?errors, "payload rejected due to validation failure");

            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Validation failed",
                    "details": errors
                })),
            )
                .into_response()
        })?;

        Ok(ValidJson(value))
    }
}

// This handler catches any request that doesn't match your defined routes
pub async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    warn!(%uri, "requested a non-existent route");
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": "Path Not Found",
            "message": format!("No route found for path: {}", uri.path())
        }))
    )
}