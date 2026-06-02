use axum::{http::StatusCode, Json};

use crate::{
    errors::AppResult,
    response::ApiResponse,
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