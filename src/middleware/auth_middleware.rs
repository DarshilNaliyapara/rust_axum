use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use diesel::query_dsl::methods::FindDsl;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use tower_cookies::Cookies;
use tracing::{error, warn};
use uuid::Uuid;

use crate::DbConn;
use crate::models::user::User;
use crate::{AppState, error::AppError, schema::users, services::jwt_create::Claims};

pub async fn auth(
    State(state): State<AppState>,
    cookies: Cookies,
    DbConn(mut conn): DbConn,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies.get("accessToken").ok_or_else(|| {
        warn!("Failed auth attempt: No access token cookie found");
        AppError::Unauthorized
    })?;

    let token = cookie.value();

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|e| {
        warn!(error = %e, "JWT validation failed for provided cookie");
        AppError::Unauthorized
    })?;

    let uuid_id =
        Uuid::parse_str(token_data.claims.sub.as_str()).map_err(|_| AppError::InvalidId)?;

    let auth_user = users::table
        .find(uuid_id)
        .first::<User>(&mut *conn)
        .await
        .map_err(|e| {
            error!("Database error fetching user {}: {:?}", uuid_id, e);
            AppError::UserNotFound
        })?;

    request.extensions_mut().insert(token_data.claims);
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}
