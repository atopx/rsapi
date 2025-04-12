use std::sync::LazyLock;

use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: i64, username: &str) -> Self {
        let exp = chrono::Local::now().naive_local().and_utc().timestamp() + KEYS.jwt_expiration;
        Self { user_id, username: username.to_string(), exp }
    }

    pub fn verify(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        decode::<Self>(token, &KEYS.decoding, &Validation::default()).map(|data| data.claims)
    }

    pub fn token(&self) -> String { encode(&Header::default(), &self, &KEYS.encoding).unwrap() }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(token_data.claims)
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
    jwt_expiration: i64,
}

impl Keys {
    fn new(secret: &[u8], jwt_exp: i64) -> Self {
        Self {
            jwt_expiration: jwt_exp,
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_exp =
        std::env::var("JWT_EXPIRATION").ok().and_then(|v| v.parse::<i64>().ok()).unwrap_or(86400);
    Keys::new(secret.as_bytes(), jwt_exp)
});
