use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Queryable, Selectable, Insertable)]
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
    pub fn new(fullname: String, username: String, email: String, password: String) -> Self {
        let now = chrono::Utc::now().naive_utc();

        Self {
            id: uuid::Uuid::now_v7(),
            full_name: fullname,
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
    pub fullname: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
