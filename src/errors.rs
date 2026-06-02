use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Product not found")]
    ProductNotFound,

    #[error("Order not found")]
    OrderNotFound,

    #[error("Insufficient stock")]
    InsufficientStock,

    #[error("Duplicate SKU")]
    DuplicateSku,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    InternalServerError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ProductNotFound => StatusCode::NOT_FOUND,
            AppError::OrderNotFound => StatusCode::NOT_FOUND,
            AppError::InsufficientStock => StatusCode::BAD_REQUEST,
            AppError::DuplicateSku => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            success: false,
            message: self.to_string(),
        });

        (status_code, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;