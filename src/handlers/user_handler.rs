use std::collections::HashMap;

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json::{Value, json};
use tracing::{error, info};
use uuid::Uuid;

use crate::models::user::{CreateUserRequest, User};
use crate::schema::users;
use crate::services::check_conflict::check_conflicts;
use crate::{AppError, DbConn, ValidJson};

pub async fn get_me() -> impl IntoResponse {
    println!("me");
    Html(format!("Hello this is ME"))
}

pub async fn get_user(
    DbConn(mut conn): DbConn,
    Path(id): Path<String>,
) -> Result<Json<User>, AppError> {
    let uuid_id = Uuid::parse_str(&id).map_err(|_| AppError::InvalidId)?;

    let user = users::table
        .find(uuid_id)
        .first::<User>(&mut *conn)
        .await
        .map_err(|e| {
            error!("Database error fetching user {}: {:?}", id, e);
            AppError::UserNotFound
        })?;

    Ok(Json(user))
}

pub async fn list_users(DbConn(mut conn): DbConn) -> Result<Json<Vec<User>>, AppError> {
    let users = users::table
        .load::<User>(&mut *conn)
        .await
        .expect("Error loading users");

    if users.is_empty() {
        return Err(AppError::UserNotFound);
    }

    Ok(Json(users))
}

pub async fn update_user(
    DbConn(mut conn): DbConn,
    Path(id): Path<String>,
    ValidJson(body): ValidJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| AppError::InvalidId)?;

    let mut checks = HashMap::new();
    checks.insert("username".to_string(), body.username.to_lowercase());
    checks.insert("email".to_string(), body.email.clone());

    check_conflicts(&mut *conn, "users", checks, Some(uuid)).await?;

    let rows_updated = diesel::update(users::table.find(uuid))
        .set((
            users::full_name.eq(body.fullname),
            users::username.eq(body.username.to_lowercase()),
            users::email.eq(body.email),
        ))
        .execute(&mut *conn)
        .await?;

    if rows_updated == 0 {
        return Err(AppError::UserNotFound);
    }
    info!(%uuid, "User Updated Successfully");
    Ok((
        StatusCode::OK,
        Json(json!({"message": "User updated successfully"})),
    ))
}

pub async fn delete_user(
    DbConn(mut conn): DbConn,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| AppError::InvalidId)?;
    let rows_deleted = diesel::delete(users::table.filter(users::id.eq(uuid)))
        .execute(&mut *conn)
        .await?;
    if rows_deleted == 0 {
        return Err(AppError::UserNotFound);
    }

    info!(%uuid, "User deleted successfully");
    Ok((
        StatusCode::OK,
        Json(json!({"message": "User deleted successfully"})),
    ))
}
