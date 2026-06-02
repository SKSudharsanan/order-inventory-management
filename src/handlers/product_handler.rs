use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    errors::{AppError, AppResult},
    models::{CreateProductRequest, Product},
    response::ApiResponse,
    state::AppState,
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<Product>>)> {
    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("Product name is required".to_string()));
    }

    if payload.sku.trim().is_empty() {
        return Err(AppError::BadRequest("SKU is required".to_string()));
    }

    if payload.price <= 0 {
        return Err(AppError::BadRequest("Price must be greater than 0".to_string()));
    }

    if payload.stock < 0 {
        return Err(AppError::BadRequest("Stock cannot be negative".to_string()));
    }

    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, sku, price, stock)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, sku, price, stock, created_at
        "#,
    )
    .bind(payload.name)
    .bind(payload.sku)
    .bind(payload.price)
    .bind(payload.stock)
    .fetch_one(&state.db)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            "Product created successfully",
            product,
        )),
    ))
}