use axum::{
    routing::{get,post},
    Router,
};

use crate::{
    handlers::{
        health_handler::health_check, 
        order_handler::{create_order,list_orders}, 
        product_handler::{create_product, list_products, get_product_by_id},
        ws_handler::ws_handler,
    },
    state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/health", get(health_check))
    .route("/products", get(list_products).post(create_product))
    .route("/orders", post(create_order).get(list_orders))
    .route("/ws", get(ws_handler))
    .route("/products/{id}", get(get_product_by_id))
    .with_state(state)
}