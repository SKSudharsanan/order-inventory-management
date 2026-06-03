use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    response::ApiResponse,
    state::AppState,
    utils::jwt::Claims,
};

pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub role: String,
}

impl AuthUser {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn is_manager(&self) -> bool {
        self.role == "manager"
    }

    pub fn is_staff(&self) -> bool {
        self.role == "staff"
    }

    pub fn can_create_orders(&self) -> bool{
        self.is_staff() || self.is_admin() || self.is_manager()
    }

    pub fn can_manage_inventory(&self) -> bool {
        self.is_admin() || self.is_manager()
    }

    pub fn can_manage_orders(&self) -> bool {
        self.is_admin() || self.is_manager()
    }
}

pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let message = match self {
            AuthError::MissingToken => "Authorization token is missing",
            AuthError::InvalidToken => "Invalid or expired token",
        };

        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<String> {
                success: false,
                message: message.to_string(),
                data: None,
            }),
        )
            .into_response()
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::MissingToken)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
            email: token_data.claims.email,
            role: token_data.claims.role,
        })
    }
}