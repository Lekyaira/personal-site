use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;
use super::roles::Roles;

/// Represents user
#[derive(Serialize, JsonSchema)]
pub(super) struct User {
    pub username: String,
    pub role: Roles,
}
