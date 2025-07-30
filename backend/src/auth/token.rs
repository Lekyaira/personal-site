use super::claims::Claims;
use crate::config::config;
use jsonwebtoken::{EncodingKey, Header, encode};

pub(super) fn create_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id,
        exp: chrono::Utc::now().timestamp() as usize + 86400, // 1 day
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config().secret.as_ref()),
    )
    .unwrap()
}
