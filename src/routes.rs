use axum::{
    routing::get,
    Router,
};

use crate::handlers::health_handler::health_check;

pub fn create_router() -> Router {
    Router::new().route("/health", get(health_check))
}