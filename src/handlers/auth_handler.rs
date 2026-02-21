use std::collections::HashMap;

use axum::{Json, http::StatusCode};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tower_cookies::{Cookie, Cookies};

use crate::AppError;
use crate::models::user::{AuthJsonResponse, CreateUserRequest, LoginUserRequest, User};
use crate::services::async_task::async_task;
use crate::services::check_conflict::check_conflicts;
use crate::services::jwt_cookie::save_to_cookie;
use crate::services::jwt_create::create_jwt;
use crate::{DbConn, ValidJson, schema::users};

pub async fn login(
    cookies: Cookies,
    DbConn(mut conn): DbConn,
    ValidJson(body): ValidJson<LoginUserRequest>,
) -> Result<(StatusCode, Json<AuthJsonResponse>), AppError> {
    let user = users::table
        .filter(users::username.eq(body.username.to_lowercase()))
        .first::<User>(&mut *conn)
        .await
        .map_err(|_| AppError::InvalidCredentials)?;

    let is_password_valid =
        bcrypt::verify(&body.password, &user.password).map_err(|_| AppError::InvalidCredentials)?;

    if !is_password_valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(user.id)?;
    let cookie = save_to_cookie(token.clone());
    cookies.add(cookie);

    let active_user = diesel::update(users::table.find(user.id))
        .set(users::is_active.eq(true))
        .get_result::<User>(&mut *conn)
        .await?;

    let response = AuthJsonResponse {
        status: "success".to_string(),
        message: "Login Successfull!!".to_string(),
        token: Some(token.to_string()),
        data: Some(active_user.into()),
    };

    Ok((StatusCode::OK, Json(response)))
}

pub async fn register(
    cookies: Cookies,
    DbConn(mut conn): DbConn,
    ValidJson(body): ValidJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<AuthJsonResponse>), AppError> {
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
        body.email.to_lowercase(),
        hashed_password,
    );

    let saved_user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut *conn)
        .await?;

    let token = create_jwt(saved_user.id)?;
    let cookie = save_to_cookie(token.clone());

    cookies.add(cookie);

    let response = AuthJsonResponse {
        status: "success".to_string(),
        message: "Resgistration Successfull!!".to_string(),
        token: Some(token.to_string()),
        data: Some(saved_user.into()),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn logout(cookies: Cookies) -> Result<(StatusCode, Json<AuthJsonResponse>), AppError> {

    cookies.get("accessToken").ok_or_else(|| {
        tracing::warn!("Failed logout attempt: No access token found");
        AppError::Unauthorized 
    })?;

    let mut removal_cookie = Cookie::new("accessToken", "");
    removal_cookie.set_path("/");

    cookies.remove(removal_cookie);

    let response = AuthJsonResponse {
        status: "success".to_string(),
        message: "Logout Successfull!!".to_string(),
        token: None,
        data: None,
    };

    Ok((StatusCode::OK, Json(response)))
}
