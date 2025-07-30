use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role")] // Must match Postgres enum name
#[sqlx(rename_all = "PascalCase")] // Must match Postgres variant case
pub enum Roles {
    Admin,
    User,
    Guest,
}
