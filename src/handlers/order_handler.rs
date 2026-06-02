use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    errors::{AppError, AppResult},
    models::{CreateOrderRequest, Order, Product},
    response::ApiResponse,
    state::AppState,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<Order>>)> {
    if payload.customer_name.trim().is_empty() {
        return Err(AppError::BadRequest("Customer name is required".to_string()));
    }

    if payload.quantity <= 0 {
        return Err(AppError::BadRequest("Quantity must be greater than 0".to_string()));
    }

    let mut tx = state.db.begin().await?;

    let product = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, sku, price, stock, created_at
        FROM products
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(payload.product_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::NotFound)?;

    if product.stock < payload.quantity {
        return Err(AppError::BadRequest("Insufficient stock".to_string()));
    }

    let new_stock = product.stock - payload.quantity;
    let total_amount = product.price * payload.quantity as i64;

    sqlx::query(
        r#"
        UPDATE products
        SET stock = $1
        WHERE id = $2
        "#,
    )
    .bind(new_stock)
    .bind(product.id)
    .execute(&mut *tx)
    .await?;

    let order = sqlx::query_as::<_, Order>(
        r#"
        INSERT INTO orders (customer_name, product_id, quantity, total_amount, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, customer_name, product_id, quantity, total_amount, status, created_at
        "#,
    )
    .bind(payload.customer_name)
    .bind(product.id)
    .bind(payload.quantity)
    .bind(total_amount)
    .bind("created")
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Order created successfully", order)),
    ))
}

pub async fn list_orders(
    State(state): State<AppState>,
) -> AppResult<(StatusCode, Json<ApiResponse<Vec<Order>>>)> {
    let orders = sqlx::query_as::<_, Order>(
        r#"
        SELECT id, customer_name, product_id, quantity, total_amount, status, created_at
        FROM orders
        ORDER BY created_at DESC
        "#,

    )
    .fetch_all(&state.db)
    .await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Orders fetched successfully",
            orders,
        )),
    ))
}