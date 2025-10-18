use super::claims::Claims;
use crate::config::config;
use jsonwebtoken::{EncodingKey, DecodingKey, Header, encode, decode, errors::Error, Validation, Algorithm };
use chrono::{DateTime, Utc};

/// Creates a JWT token
/// # Arguments
/// - `user_id`: `i32` - user ID (from database)
/// - `expires`: `chrono::DateTime<Utc>` - Token expiration date
pub(super) fn create_jwt(user_id: i32, expires: DateTime<Utc>) -> Result<String, Error> {
    let claims = Claims {
        sub: user_id,
        exp: expires.timestamp() as usize,
    };
    println!("Create jwt: {:?}", claims);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config().secret.as_ref()),
    )
}

/// Returns the `auth::Claims` encoded in the JWT
pub(super) fn get_claims(token: String) -> Result<Claims, Error> {
    Ok(decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config().secret.as_ref()),
        &Validation::default(),
    )?.claims)
}
