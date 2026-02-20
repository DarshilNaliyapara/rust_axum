use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::{debug, error, info};

use crate::DbConn;
use crate::models::user::User;
use crate::schema::users;

pub async fn get_me() -> impl IntoResponse {
    println!("me");
    Html(format!("Hello this is ME"))
}

pub async fn get_user(Path(id): Path<String>) -> impl IntoResponse {
    println!("user with id: {id}");

    Html(format!("User with id: {id}"))
}

pub async fn list_users(DbConn(mut conn): DbConn) -> impl IntoResponse {
    let users = users::table
        .load::<User>(&mut *conn)
        .await
        .expect("Error loading users");

    (StatusCode::OK, Json(users)).into_response()
}

pub async fn update_user() {
    println!("update user");
}

pub async fn delete_user(DbConn(mut conn): DbConn, Path(id): Path<String>) -> impl IntoResponse {
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UUID format").into_response(),
    };

    let result = diesel::delete(users::table.filter(users::id.eq(uuid)))
        .execute(&mut *conn)
        .await;

    match result {
        Ok(0) => {
            // Log as debug or warn, since it's a client error (target didn't exist)
            debug!("Attempted to delete a non-existent user");
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "User not found"})),
            )
                .into_response()
        }
        Ok(_) => {
            // Log successful state changes
            info!("User deleted successfully");
            (
                StatusCode::OK,
                Json(serde_json::json!({"message": "User deleted successfully"})),
            )
                .into_response()
        }
        Err(e) => {
            // Production-grade server error logging
            error!(error = %e, "Failed to execute user deletion query");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Database error"})),
            )
                .into_response()
        }
    }
}
