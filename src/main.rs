mod config;
mod errors;
mod handlers;
mod models;
mod repositories;
mod response;
mod routes;
mod state;
mod utils;
mod middleware;

use config::Config;
use routes::create_router;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::sync::broadcast;
use tower_http::{cors::CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
   tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();
    let config = Config::from_env();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to postgres");

    let (event_tx, _) = broadcast::channel(100);

    let state = AppState { db, event_tx, jwt_secret: config.jwt_secret, jwt_expiration_hours: config.jwt_expiration_hours, };
    let app = create_router(state)
    .layer(CorsLayer::permissive())
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );
    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .expect("failed to bind server");

    tracing::info!("server running on http://{}", config.server_addr);

    axum::serve(listener, app).await.expect("server failed");
}
