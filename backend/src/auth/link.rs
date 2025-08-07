use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;

/// Represents a navigation link
#[derive(Serialize, JsonSchema)]
pub struct Link {
    pub name: String,
    pub to: String,
}
