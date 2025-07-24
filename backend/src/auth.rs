use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

fn create_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id,
        exp: chrono::Utc::now().timestamp() as usize + 86400, // 1 day
    };
    let secret = std::env::var("JWT_SECRET").expect("Missing JWT_SECRET");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
