use crate::config::CONFIG;
use crate::error::{Error, Result};
use axum::body::Body;
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    // exp: usize, NOTE: support in the future
}

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, Error> {
    let auth_header = req.headers().get("Authorization").and_then(|hv| hv.to_str().ok());

    if let Some(auth_header) = auth_header {
        if !auth_header.starts_with("Bearer ") {
            return Err(Error::Unauthorized("Invalid token".to_string()));
        }
        let token = auth_header.trim_start_matches("Bearer ").trim();

        let secret = CONFIG.jwt_secret.clone();
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
            Ok(token_data) => {
                tracing::debug!("Valid token for user: {:?}", token_data.claims.sub);
                Ok(next.run(req).await)
            }
            Err(err) => {
                tracing::warn!("Token validation error: {}", err);
                Err(Error::Unauthorized("Invalid token".to_string()))
            }
        }
    } else {
        Err(Error::Unauthorized("Invalid token".to_string()))
    }
}
