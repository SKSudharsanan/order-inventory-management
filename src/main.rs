mod handlers;
mod errors;
mod response;
mod routes;
mod state;
mod models;
mod repositories;
mod config;

use routes::create_router;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::sync::broadcast;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await
    .expect("failed to connect to postgres");

    let (event_tx, _) = broadcast::channel(100);

let state = AppState {
    db,
    event_tx,
};
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .expect("failed to bind server");

    println!("server running on http://{}", config.server_addr);

    axum::serve(listener, app)
        .await
        .expect("server failed");
}