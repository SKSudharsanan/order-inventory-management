use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use serde_json;

use crate::{
    errors::{AppError, AppResult},
    models::{CreateProductRequest, Product},
    response::ApiResponse,
    state::AppState,
    repositories::product_repository
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

let product = product_repository::create_product(&state.db, payload).await?;
    let event = serde_json::json!({
    "event": "product_created",
    "product_id": product.id,
    "name": product.name,
    "stock": product.stock
});

let _ = state.event_tx.send(event.to_string());

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            "Product created successfully",
            product,
        )),
    ))
}

pub async fn list_products(
    State(state): State<AppState>,
) -> AppResult<(StatusCode, Json<ApiResponse<Vec<Product>>>)> {
    let products = product_repository::list_products(&state.db).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Products fetched successfully",
            products,
        )),
    ))
}