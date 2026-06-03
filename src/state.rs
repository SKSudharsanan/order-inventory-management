use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub event_tx: broadcast::Sender<String>,
    pub jwt_secret: String,
}