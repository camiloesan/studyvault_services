use actix_web::Error;
use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_web::error::ErrorUnauthorized;
use serde::{Deserialize, Serialize};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::dev::ServiceRequest;
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i32,
    exp: usize,
}

pub fn generate_jwt(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY not in .env");

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs() as usize
        + 7200; // Expire in 2 hrs

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

pub async fn validate_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    dotenv().ok();

    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY not in .env");

    let token = credentials.token();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(_) => Ok(req),
        Err(_) => Err(ErrorUnauthorized("Invalid token")),
    }
}