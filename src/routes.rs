use axum::{
    routing::{get,post},
    Router,
};

use crate::{
    handlers::{
        health_handler::health_check,
        product_handler::{create_product, list_products},
    },
    state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/health", get(health_check))
    .route("/products", get(list_products).post(create_product))
    .with_state(state)
}