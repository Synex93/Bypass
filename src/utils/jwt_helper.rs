use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

use crate::error::AppError;

static JWT_SECRET: OnceLock<String> = OnceLock::new();

fn get_jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| env::var("jwt_secret").expect("jwt secret must be set"))
}

// Define the claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i64,
    pub email: String,
    exp: usize, // Expiration time (as UTC timestamp)
}

pub fn create_jwt(id: i64, email: &str) -> Result<String, AppError> {
    // Define some claims
    let my_claims = Claims {
        id,
        email: email.to_string(),
        exp: (Utc::now() + Duration::hours(168)).timestamp() as usize, // JWT expires in 168 hours
    };

    // Encoding the token
    let token = encode(
        &Header::new(Algorithm::HS256),
        &my_claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )?;

    // todo: write cache

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    // todo: check cache

    Ok(token_data)
}
