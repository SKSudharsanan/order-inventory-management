use axum::{Json, extract::{State, Path}, http::StatusCode};

use serde_json;

use crate::{
    errors::{AppError, AppResult},
    models::{CreateProductRequest, Product, UpdateStockRequest},
    repositories::product_repository,
    response::ApiResponse,
    state::AppState,
    middleware::AuthUser
};

pub async fn create_product(
     _auth_user: AuthUser,
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
        return Err(AppError::BadRequest(
            "Price must be greater than 0".to_string(),
        ));
    }

    if payload.stock < 0 {
        return Err(AppError::BadRequest("Stock cannot be negative".to_string()));
    }
    let product = match product_repository::create_product(&state.db, payload).await {
        Ok(product) => product,
        Err(sqlx::Error::Database(db_error)) => {
            if db_error.constraint() == Some("products_sku_key") {
                return Err(AppError::DuplicateSku);
            }
            return Err(AppError::DatabaseError(sqlx::Error::Database(db_error)));
        }
        Err(error) => return Err(AppError::DatabaseError(error)),
    };
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

pub async fn get_product_by_id(
    State(state): State<AppState>,
    Path(product_id): Path<uuid::Uuid>,
) -> AppResult<(StatusCode, Json<ApiResponse<Product>>)> {
    let product = product_repository::find_product_by_id(&state.db, product_id)
        .await?
        .ok_or(AppError::ProductNotFound)?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Product fetched successfully",
            product,
        )),
    ))
}

pub async fn update_product_stock(
     _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(product_id): Path<uuid::Uuid>,
    Json(payload): Json<UpdateStockRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<Product>>)> {
    if payload.stock < 0 {
        return Err(AppError::BadRequest("Stock cannot be negative".to_string()));
    }

    let product = product_repository::update_product_stock(
        &state.db,
        product_id,
        payload.stock,
    )
    .await?
    .ok_or(AppError::ProductNotFound)?;

    let event = serde_json::json!({
        "event": "product_stock_updated",
        "product_id": product.id,
        "name": product.name,
        "stock": product.stock
    });

    let _ = state.event_tx.send(event.to_string());

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Product stock updated successfully",
            product,
        )),
    ))
}

