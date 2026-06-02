use axum::{
    routing::get,
    Router,
};

use crate::{handlers::health_handler::health_check, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
    .route("/health", get(health_check))
    .with_state(state)
}