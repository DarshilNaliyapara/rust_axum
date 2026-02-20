use axum::{Json, http::StatusCode, response::IntoResponse};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::{DbConn, schema::users , ValidJson};
use crate::models::user::{CreateUserRequest, User};

pub async fn login() {
    println!("login");
}

pub async fn register(
    DbConn(mut conn): DbConn,
    ValidJson(body): ValidJson<CreateUserRequest>,
) -> impl IntoResponse {

    let hashed_password = bcrypt::hash(body.password, bcrypt::DEFAULT_COST).unwrap();
    let new_user = User::new(
        body.fullname,
        body.username.to_lowercase(),
        body.email,
        hashed_password,
    );

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut *conn)
        .await;

    match insert_result {
        Ok(saved_user) => (StatusCode::CREATED, Json(saved_user)).into_response(),
        Err(e) => {
            eprintln!("Failed to insert user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to save user to database: {e}")})),
            )
                .into_response()
        }
    }
}

pub async fn logout() {
    println!("logout");
}
