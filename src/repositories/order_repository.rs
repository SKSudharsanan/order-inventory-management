use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::{Order, Product};

pub async fn find_product_for_update(
    tx: &mut Transaction<'_, Postgres>,
    product_id: Uuid,
) -> Result<Option<Product>, sqlx::Error> {
    sqlx::query_as::<_, Product>(
        r#"
        SELECT id, name, sku, price, stock, created_at
        FROM products
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(product_id)
    .fetch_optional(&mut **tx)
    .await
}

pub async fn update_product_stock(
    tx: &mut Transaction<'_, Postgres>,
    product_id: Uuid,
    new_stock: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE products
        SET stock = $1
        WHERE id = $2
        "#,
    )
    .bind(new_stock)
    .bind(product_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_order(
    tx: &mut Transaction<'_, Postgres>,
    customer_name: String,
    product_id: Uuid,
    quantity: i32,
    total_amount: i64,
) -> Result<Order, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        INSERT INTO orders (customer_name, product_id, quantity, total_amount, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, customer_name, product_id, quantity, total_amount, status, created_at
        "#,
    )
    .bind(customer_name)
    .bind(product_id)
    .bind(quantity)
    .bind(total_amount)
    .bind("created")
    .fetch_one(&mut **tx)
    .await
}

pub async fn list_orders(db: &PgPool) -> Result<Vec<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        SELECT id, customer_name, product_id, quantity, total_amount, status, created_at
        FROM orders
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(db)
    .await
}

pub async fn find_order_by_id(
    db: &PgPool,
    order_id: Uuid,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        SELECT id, customer_name, product_id, quantity, total_amount, status, created_at
        FROM orders
        WHERE id = $1
        "#,
    )
    .bind(order_id)
    .fetch_optional(db)
    .await
}

pub async fn update_order_status(
    db: &PgPool,
    order_id: Uuid,
    status: String,
) -> Result<Option<Order>, sqlx::Error> {
    sqlx::query_as::<_, Order>(
        r#"
        UPDATE orders
        SET status = $1
        WHERE id = $2
        RETURNING id, customer_name, product_id, quantity, total_amount, status, created_at
        "#,
    )
    .bind(status)
    .bind(order_id)
    .fetch_optional(db)
    .await
}