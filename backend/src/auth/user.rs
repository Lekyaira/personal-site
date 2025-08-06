use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;
use super::roles::Roles;

#[derive(Serialize, JsonSchema)]
pub(super) struct User {
    pub username: String,
    pub role: Roles,
}
