use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    errors::{AppError, AppResult},
    middleware::AuthUser,
    models::{User, UserResponse},
    repositories::user_repository,
    response::ApiResponse,
    state::AppState,
};

pub async fn list_users(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> AppResult<(StatusCode, Json<ApiResponse<Vec<UserResponse>>>)> {
    if !auth_user.is_admin() {
        return Err(AppError::Forbidden);
    }

    let users = user_repository::list_users(&state.db).await?;

    let users_response: Vec<UserResponse> = users
        .into_iter()
        .map(UserResponse::from)
        .collect();

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Users fetched successfully",
            users_response,
        )),
    ))
}