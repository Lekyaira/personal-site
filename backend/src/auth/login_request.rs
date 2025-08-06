// use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Deserialize;

/// Represents a log in request
#[derive(JsonSchema, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub stay_logged_in: bool,
}
