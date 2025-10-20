use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;

/// Represents user
#[derive(Serialize, JsonSchema)]
pub(super) struct User {
    pub email: String,
    pub username: String,
}
