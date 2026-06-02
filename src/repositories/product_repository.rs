use sqlx::PgPool;

use crate::models::{CreateProductRequest, Product};

pub async fn create_product(
    db: &PgPool,
    payload: CreateProductRequest,
) -> Result<Product, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, sku, price, stock)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, sku, price, stock, created_at
        "#,
    )
    .bind(payload.name)
    .bind(payload.sku)
    .bind(payload.price)
    .bind(payload.stock)
    .fetch_one(db)
    .await
}

pub async fn list_products(db: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, sku, price, stock, created_at
        FROM products
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(db)
    .await
}