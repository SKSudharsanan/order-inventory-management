use axum::{
    routing::{get,post, patch},
    Router,
};

use crate::{
    handlers::{
        health_handler::{
            health_check,
            readiness_check
        }, 
        auth_handler::{
            register_user,
            login_user
        },
        order_handler::{
            create_order,
            list_orders, 
            get_order_by_id,
            update_order_status
        }, 
        product_handler::{
            create_product, 
            list_products,
            get_product_by_id,
            update_product_stock},
        ws_handler::ws_handler,
    },
    state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/ready", get(readiness_check))
    .route("/health", get(health_check))
    .route("/auth/register", post(register_user))
    .route("/auth/login", post(login_user))
    .route("/products", get(list_products).post(create_product))
    .route("/orders", post(create_order).get(list_orders))
    .route("/ws", get(ws_handler))
    .route("/products/{id}", get(get_product_by_id))
    .route("/products/{id}/stock", patch(update_product_stock))
    .route("/orders/{id}", get(get_order_by_id))
    .route("/orders/{id}/status", patch(update_order_status))
    .with_state(state)
}