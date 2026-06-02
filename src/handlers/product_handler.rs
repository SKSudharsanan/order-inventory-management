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
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, sku, price, stock, created_at
        FROM products
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Products fetched successfully",
            products,
        )),
    ))
}