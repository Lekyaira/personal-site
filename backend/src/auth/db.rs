use super::auth_user::AuthUser;
use super::login_request::LoginRequest;
use super::password::hash_password;
use super::roles::Roles;
use crate::db::UserDB;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{Connection, sqlx::Row};

/// Returns `Ok` if the authorized user matches the given `Role` or better
/// # Arguments
/// - `user`: `auth::AuthUser` - Rocket guard
/// - `access_level`: `auth::Roles` - Role to match against
/// - `mut db`: `Connection<UserDB>` - Rocket Sqlx_pools DB
/// # Returns
/// - `Result<(), Status>`
pub async fn authorize_role(
    user: AuthUser,
    access_level: Roles,
    mut db: Connection<UserDB>,
) -> Result<(), Status> {
    let row = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(&user.0)
        .fetch_one(&mut **db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let role: Roles = row.get("role");
    role.authorize(access_level)?;

    Ok(())
}

pub(super) async fn create_user(
    req: Json<LoginRequest>,
    db: &mut Connection<UserDB>,
) -> Result<i32, Status> {
    // Hash the password before adding it to the database
    let password = hash_password(&req.password);

    // Insert the user into the database
    let row = sqlx::query(
        "INSERT INTO users (username, password, role) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&req.username)
    .bind(password)
    .bind(Roles::User)
    .fetch_one(db.as_mut())
    .await
    .map_err(|_| Status::InternalServerError)?; // TODO: Parse errors, tell client if user
    // exists

    Ok(row.get("id"))
}
