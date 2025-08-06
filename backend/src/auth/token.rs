use super::claims::Claims;
use crate::config::config;
use jsonwebtoken::{EncodingKey, DecodingKey, Header, encode, decode, errors::Error, Validation};
use chrono::{DateTime, Utc};

pub(super) fn create_jwt(user_id: i32, expires: DateTime<Utc>) -> Result<String, Error> {
    let claims = Claims {
        sub: user_id,
        exp: expires,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config().secret.as_ref()),
    )
}

pub(super) fn get_claims(token: String) -> Result<Claims, Error> {
    Ok(decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config().secret.as_ref()),
        &Validation::default(),
    )?.claims)
}
