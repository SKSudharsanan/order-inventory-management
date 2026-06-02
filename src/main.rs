mod handlers;
mod errors;
mod response;
mod routes;
mod state;
mod models;

use routes::create_router;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let server_url= std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("failed to connect to postgres");

    let (event_tx, _) = broadcast::channel(100);

let state = AppState {
    db,
    event_tx,
};
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&server_url)
        .await
        .expect("failed to bind server");

    println!("server running on http://{}", server_url);

    axum::serve(listener, app)
        .await
        .expect("server failed");
}