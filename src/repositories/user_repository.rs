use sqlx::PgPool;

use crate::models::User;
use uuid::Uuid;

pub async fn create_user(
    db: &PgPool,
    username: String,
    email: String,
    password_hash: String,
    role: String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, password_hash, role, created_at
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .fetch_one(db)
    .await
}

pub async fn find_user_by_email(
    db: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, email, password_hash, role, created_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(db)
    .await
}

pub async fn find_user_by_id(
    db: &PgPool,
    user_id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, email, password_hash, role, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await
}