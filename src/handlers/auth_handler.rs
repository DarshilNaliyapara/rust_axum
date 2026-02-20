use std::collections::HashMap;

use axum::{Json, http::StatusCode};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use tracing::info;

use crate::AppError;
use crate::models::user::{CreateUserRequest, User};
use crate::services::async_task::async_task;
use crate::services::check_conflict::check_conflicts;
use crate::{DbConn, ValidJson, schema::users};

pub async fn login() {
    println!("=======================login===============================");
}

pub async fn register(
    DbConn(mut conn): DbConn,
    ValidJson(body): ValidJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    
    let mut checks = HashMap::new();
    checks.insert("username".to_string(), body.username.to_lowercase());
    checks.insert("email".to_string(), body.email.clone());

    check_conflicts(&mut *conn, "users", checks, None).await?;

    let hashed_password = async_task(move || bcrypt::hash(body.password, bcrypt::DEFAULT_COST))
        .await?
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to hash password");
            AppError::InternalServerError
        })?;

    let new_user = User::new(
        body.fullname,
        body.username.to_lowercase(),
        body.email,
        hashed_password,
    );

    let saved_user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut *conn)
        .await?;

    info!(username = %saved_user.username, "New user registered successfully");
    Ok((StatusCode::CREATED, Json(saved_user)))
}

pub async fn logout() {
    println!("logout");
}
