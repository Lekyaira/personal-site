use crate::config::config;
use crate::db::BlogDB;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::json::Json,
};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{self, Row};
use rocket_okapi::{
    okapi::{schemars, schemars::JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, JsonSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String, // Don't return this in responses
}

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
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(str::to_string);

        if let Some(token) = token {
            let result = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(config().secret.as_ref()),
                &Validation::default(),
            );

            if let Ok(data) = result {
                return Outcome::Success(AuthUser(data.claims.sub));
            }
        }

        Outcome::Error((Status::Unauthorized, Status::Unauthorized))
    }
}

#[derive(JsonSchema, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[openapi]
#[post("/login", data = "<req>")]
pub async fn login(
    req: Json<LoginRequest>,
    mut db: Connection<BlogDB>,
) -> Result<Json<String>, Status> {
    let row = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(&req.username)
        .fetch_one(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let user = User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password"),
    };

    if verify_password(&user.password_hash, &req.password) {
        let token = create_jwt(user.id);
        return Ok(Json(token));
    }

    Err(Status::Unauthorized)
}
