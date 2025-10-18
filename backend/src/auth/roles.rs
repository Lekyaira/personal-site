use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket::http::Status;
use std::cmp::Ordering;

/// User roles, used for authenticated access to certain endpoints
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, JsonSchema)]
#[sqlx(type_name = "roles")] // Must match Postgres enum name
#[sqlx(rename_all = "PascalCase")] // Must match Postgres variant case
pub enum Roles {
    Admin,
    User,
    Guest,
}

impl Roles {
    pub fn authorize(&self, comp_role: Self) -> Result<(), Status> {
        if comp_role < *self {
            return Err(Status::Unauthorized);
        }
        Ok(())
    }
}

impl Ord for Roles {
    fn cmp(&self, other: &Self) -> Ordering {
        fn rank(role: &Roles) -> u8 {       // Explicitly rank in case order changes
            match role {
                Roles::Guest => 0,
                Roles::User => 1,
                Roles::Admin => 2,
            }
        }
        rank(self).cmp(&rank(other))
    }
}

impl PartialOrd for Roles {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
