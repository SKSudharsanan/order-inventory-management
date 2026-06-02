use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub sku: String,
    pub price: i64,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub sku: String,
    pub price: i64,
    pub stock: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]

pub struct Order {
    pub id: Uuid,
    pub customer_name: String,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total_amount: i64,
    pub status: String,
    pub created_at: DateTime<Utc>,

}

#[derive(Debug, Deserialize)]

pub struct CreateOrderRequest {
    pub customer_name: String,
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]

pub struct UpdateStockRequest {
    pub stock: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}