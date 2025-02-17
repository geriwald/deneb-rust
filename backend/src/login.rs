use crate::auth;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String, // For demonstration only.
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    // ...validate credentials here...
    match auth::create_jwt(&payload.username, &payload.password) {
        Ok(token) => Ok(Json(LoginResponse { token })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
