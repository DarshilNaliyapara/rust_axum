use axum::Json;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json::{Value, json};
use tracing::info;

use crate::models::user::User;
use crate::schema::users;
use crate::{AppError, DbConn};

pub async fn get_me() -> impl IntoResponse {
    println!("me");
    Html(format!("Hello this is ME"))
}

pub async fn get_user(Path(id): Path<String>) -> impl IntoResponse {
    println!("user with id: {id}");

    Html(format!("User with id: {id}"))
}

pub async fn list_users(DbConn(mut conn): DbConn) -> Result<Json<Vec<User>>, AppError> {
    let users = users::table
        .load::<User>(&mut *conn)
        .await
        .expect("Error loading users");
    
    Ok(Json(users))
}

pub async fn update_user() {
    println!("update user");
}

pub async fn delete_user(
    DbConn(mut conn): DbConn,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| AppError::InvalidId)?;
    let rows_deleted = diesel::delete(users::table.filter(users::id.eq(uuid)))
        .execute(&mut *conn)
        .await?;
    if rows_deleted == 0 {
        return Err(AppError::UserNotFound);
    }

    info!(%uuid, "User deleted successfully");
    Ok(Json(json!({"message": "User deleted successfully"})))
}
