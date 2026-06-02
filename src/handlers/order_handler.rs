use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    errors::{AppError, AppResult},
    models::{CreateOrderRequest, Order},
    repositories::order_repository,
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
        return Err(AppError::BadRequest(
            "Quantity must be greater than 0".to_string(),
        ));
    }

    let mut tx = state.db.begin().await?;

    let product = order_repository::find_product_for_update(
        &mut tx,
        payload.product_id,
    )
    .await?
    .ok_or(AppError::ProductNotFound)?;

    if product.stock < payload.quantity {
        return Err(AppError::InsufficientStock);
    }

    let new_stock = product.stock - payload.quantity;
    let total_amount = product.price * payload.quantity as i64;

    order_repository::update_product_stock(
        &mut tx,
        product.id,
        new_stock,
    )
    .await?;

    let order = order_repository::create_order(
        &mut tx,
        payload.customer_name,
        product.id,
        payload.quantity,
        total_amount,
    )
    .await?;

    tx.commit().await?;

    let event = serde_json::json!({
        "event": "order_created",
        "order_id": order.id,
        "product_id": product.id,
        "quantity": order.quantity,
        "remaining_stock": new_stock
    });

    let _ = state.event_tx.send(event.to_string());

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Order created successfully", order)),
    ))
}

pub async fn list_orders(
    State(state): State<AppState>,
) -> AppResult<(StatusCode, Json<ApiResponse<Vec<Order>>>)> {
    let orders = order_repository::list_orders(&state.db).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Orders fetched successfully",
            orders,
        )),
    ))
}