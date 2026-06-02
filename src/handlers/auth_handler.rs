use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    errors::{AppError, AppResult},
    models::{RegisterUserRequest, User, UserResponse, LoginUserRequest},
    repositories::user_repository,
    response::ApiResponse,
    state::AppState,
    utils::password::{hash_password, verify_password},
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UserResponse>>)> {
    if payload.username.trim().is_empty() {
        return Err(AppError::BadRequest("Username is required".to_string()));
    }

    if payload.email.trim().is_empty() {
        return Err(AppError::BadRequest("Email is required".to_string()));
    }

    if payload.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    let role = payload.role.unwrap_or_else(|| "staff".to_string());

    let valid_roles = ["admin", "manager", "staff"];

    if !valid_roles.contains(&role.as_str()) {
        return Err(AppError::BadRequest("Invalid user role".to_string()));
    }

    let password_hash = hash_password(&payload.password)
        .map_err(|_| AppError::InternalServerError)?;

    let user = user_repository::create_user(
        &state.db,
        payload.username,
        payload.email,
        password_hash,
        role,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            "User registered successfully",
            user.into(),
        )),
    ))
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<UserResponse>>)> {
    if payload.email.trim().is_empty() {
        return Err(AppError::BadRequest("Email is required".to_string()));
    }

    if payload.password.trim().is_empty() {
        return Err(AppError::BadRequest("Password is required".to_string()));
    }

    let user = user_repository::find_user_by_email(&state.db, &payload.email)
        .await?
        .ok_or(AppError::BadRequest("Invalid email or password".to_string()))?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError)?;

    if !is_valid {
        return Err(AppError::BadRequest("Invalid email or password".to_string()));
    }

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Login successful",
            user.into(),
        )),
    ))
}