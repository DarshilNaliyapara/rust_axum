use std::env;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::error::AppError; // Adjust to your actual AppError path

// The payload that goes inside the token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize, 
}

pub fn create_jwt(user_id: Uuid) -> Result<String, AppError> {
    let now = Utc::now();
    
    let expiration = now.checked_add_signed(Duration::minutes(30))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: now.timestamp() as usize,
    };

    let secret = env::var("ACCESS_SECRET").map_err(|_| {
        tracing::error!("FATAL: ACCESS_SECRET environment variable is not set");
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