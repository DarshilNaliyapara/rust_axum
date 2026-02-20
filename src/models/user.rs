
use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::users;
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
            created_at: now,
            updated_at: None, // Standardized: Always None on creation
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    // Intent: Ensure full name is not empty
    #[validate(length(min = 3, message = "Full name is required"))]
    pub fullname: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
