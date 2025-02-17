use axum::{http::Request, http::StatusCode, middleware::Next, response::Response};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(sub: &str, password: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let expiration = (Utc::now() + Duration::seconds(3600)).timestamp() as usize;
    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub async fn jwt_auth<B>(req: Request<B>, next: Next) -> Result<Response, StatusCode>
where
    B: Into<axum::body::Body>,
{
    // Bypass authentication for /login page
    if req.uri().path() == "/login" {
        return Ok(next.run(req.map(Into::into)).await);
    }

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    if let Some(token) = token {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
        match verify_jwt(&token, &secret) {
            Ok(_claims) => Ok(next.run(req.map(Into::into)).await),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
