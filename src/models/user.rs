use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(full_name: String, username: String, email: String, password: String) -> Self {
        let now = chrono::Utc::now().naive_utc();

        Self {
            id: uuid::Uuid::now_v7(),
            full_name: full_name,
            username,
            email,
            password,
            is_admin: false,
            is_active: true,
            created_at: now,
            updated_at: None,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "Full name is required"))]
    pub full_name: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(length(min = 3, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthJsonResponse {
    pub status: String,
    pub message: String,
    pub token: Option<Vec<String>>,
    pub data: Option<UserResponse>,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            is_admin: user.is_admin,
            is_active: user.is_active,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub old_password: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
}

#[derive(Deserialize, AsChangeset, Validate)]
#[diesel(table_name = users)]
pub struct UpdateUserPayload {
    pub full_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}
