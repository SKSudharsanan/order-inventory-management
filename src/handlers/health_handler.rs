use axum::{extract::State, http::StatusCode, Json};

use crate::{
    errors::AppResult,
    response::ApiResponse,
    state::AppState
};

pub async fn health_check() -> AppResult<(StatusCode, Json<ApiResponse<String>>)> {
    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Server is running",
            "OK".to_string(),
        )),
    ))
}

pub async fn readiness_check(

    State(state): State<AppState>,

) -> AppResult<(StatusCode, Json<ApiResponse<String>>)> {

    sqlx::query("SELECT 1")

        .execute(&state.db)

        .await?;

    Ok((

        StatusCode::OK,

        Json(ApiResponse::success(

            "Database connection is healthy",

            "READY".to_string(),

        )),

    ))

}