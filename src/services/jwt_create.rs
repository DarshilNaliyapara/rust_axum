use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::error::AppError; // Adjust to your actual AppError path

// The payload that goes inside the token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (The User ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at time
}

pub fn create_jwt(user_id: Uuid) -> Result<String, AppError> {
    let now = Utc::now();
    
    let expiration = now.checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now.timestamp() as usize,
    };

    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        tracing::error!("FATAL: JWT_SECRET environment variable is not set");
        AppError::InternalServerError
    })?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).map_err(|e| {
        tracing::error!(error = %e, "Failed to encode JWT");
        AppError::InternalServerError
    })
}