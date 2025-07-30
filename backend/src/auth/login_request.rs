// use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Deserialize;

#[derive(JsonSchema, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
