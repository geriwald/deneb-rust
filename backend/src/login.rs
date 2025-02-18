use crate::auth;
use axum::body::Body;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
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

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // ...validate credentials here...
    if payload.username != "username" || payload.password != "password" {
        let mut response: axum::response::Response<Body> =
            axum::response::Response::new("Login failed".into());
        *response.status_mut() = StatusCode::UNAUTHORIZED;
        return response;
    }
    // ...authentification réussie, génération du token...
    let token = auth::create_jwt(&payload.username).unwrap();
    let cookie = format!(
        "Authorization={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600",
        token
    );
    let mut response = axum::response::Response::new("Login success".into());
    response
        .headers_mut()
        .insert("Set-Cookie", HeaderValue::from_str(&cookie).unwrap());
    response
}
