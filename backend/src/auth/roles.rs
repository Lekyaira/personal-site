use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

/// User roles, used for authenticated access to certain endpoints
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, JsonSchema)]
#[sqlx(type_name = "user_role")] // Must match Postgres enum name
#[sqlx(rename_all = "PascalCase")] // Must match Postgres variant case
pub enum Roles {
    Admin,
    User,
    Guest,
}
