use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{debug, error};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database execution error")]
    Database(#[from] diesel::result::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Invalid UUID format")]
    InvalidId,

    #[error("Conflict on fields: {0:?}")]
    Conflict(Vec<String>),

    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error{"Restricted"}]
    Restricted,
}

// Tell Axum how to convert these errors into HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(diesel::result::Error::NotFound) => {
                debug!("Database query returned no results");
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }

            AppError::Conflict(conflicting_fields) => {
                tracing::warn!(
                    ?conflicting_fields,
                    "Client attempted to register with existing unique fields"
                );

                let body = Json(serde_json::json!({
                    "error": "Conflict",
                    "fields": conflicting_fields,
                    "message": "These field's value are already in use"
                }));

                return (StatusCode::CONFLICT, body).into_response();
            }

            AppError::Database(e) => {
                error!(error = %e, "Database query failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal database error".to_string(),
                )
            }

            AppError::UserNotFound => {
                debug!("Attempted to access a non-existent user");
                (StatusCode::NOT_FOUND, "User not found".to_string())
            }

            AppError::InternalServerError => {
                error!("An unexpected internal server error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::InvalidId => {
                debug!("Client provided an invalid UUID string");
                (StatusCode::BAD_REQUEST, "Invalid ID format".to_string())
            }

            AppError::InvalidCredentials => {
                debug!("Invalid credentials provided");
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }

            AppError::Unauthorized => {
                debug!("Unauthorized access attempt");
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
            }

            AppError::Restricted => {
                debug!("Restricted access attempt");
                (StatusCode::FORBIDDEN, "Restricted".to_string())
            }
        };

        let body = Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(inner: anyhow::Error) -> Self {
        tracing::error!("Internal Server Error (Anyhow): {:?}", inner);
        AppError::InternalServerError 
    }
}