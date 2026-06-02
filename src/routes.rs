use axum::{
    routing::{get,post},
    Router,
};

use crate::{
    handlers::{
        health_handler::health_check, order_handler::create_order, product_handler::{create_product, list_products}
    },
    state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/health", get(health_check))
    .route("/products", get(list_products).post(create_product))
    .route("/orders", post(create_order))
    .with_state(state)
}