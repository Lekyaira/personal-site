use crate::config::config;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
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
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config().secret.as_ref()),
    )
    .unwrap()
}

/// Authorizes a user for endpoint request
///
/// # Example
/// ```rust
/// // Protected Rocket endpoint
/// #[get("/me")]
/// async fn me(user: AuthUser) -> Json<i32> {
///     Json(user.0) // user id
/// }
/// ```
pub struct AuthUser(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(str::to_string);

        if let Some(token) = token {
            let secret = config().secret;
            let result = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            );

            if let Ok(data) = result {
                return Outcome::Success(AuthUser(data.claims.sub));
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
