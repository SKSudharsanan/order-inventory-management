use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind server");

    println!("server running on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn health_check() -> &'static str {
    "OK"
}