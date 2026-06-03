use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub fn generate_token(
    user_id: Uuid,
    email: String,
    role: String,
    secret: &str,
    expiration_hours: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::hours(expiration_hours))
    .expect("valid timestamp")
    .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email,
        role,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}