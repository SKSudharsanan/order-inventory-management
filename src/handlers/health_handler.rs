use axum::{http::StatusCode, Json};

use crate::response::ApiResponse;

pub async fn health_check() -> (StatusCode, Json<ApiResponse<String>>) {
    (
        StatusCode::OK,
        Json(ApiResponse::success(
            "Server is running",
            "OK".to_string(),
        )),
    )
}