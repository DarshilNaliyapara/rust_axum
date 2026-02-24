use axum::{Extension, extract::Request, middleware::Next, response::IntoResponse};

use crate::{AppError, models::user::User};

pub async fn is_admin(
    Extension(auth_user): Extension<User>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    if auth_user.is_admin {
        Ok(next.run(request).await)
    } else {
        Err(AppError::Restricted)
    }
}
