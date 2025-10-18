use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;

/// Represents user
#[derive(Serialize, JsonSchema)]
pub(super) struct User {
    pub username: String,
    pub callby: Option<String>,
    pub fullname: Option<String>,
}
