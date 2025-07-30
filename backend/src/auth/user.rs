use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub(super) struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String, // Don't return this in responses
}
