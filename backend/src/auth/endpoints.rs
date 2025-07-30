use super::auth_user::AuthUser;
use super::db::create_user;
use super::login_request::LoginRequest;
use super::password::verify_password;
use super::roles::Roles;
use super::token::create_jwt;
use super::user::User;
use crate::db::UserDB;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{Connection, sqlx::Row};
use rocket_okapi::openapi;

fn map_db_err(e: sqlx::Error) -> Status {
    use std::io::ErrorKind;

    match &e {
        // No matching row for SELECT … FETCH_ONE
        sqlx::Error::RowNotFound => Status::NotFound,

        // Couldn’t reach the DB / pool timed out
        sqlx::Error::Io(io) => {
            if io.kind() == ErrorKind::TimedOut {
                Status::ServiceUnavailable
            } else {
                Status::InternalServerError
            }
        }
        sqlx::Error::PoolTimedOut => Status::ServiceUnavailable,
        sqlx::Error::PoolClosed => Status::ServiceUnavailable,

        // Anything else
        _ => Status::InternalServerError,
    }
}

#[openapi]
#[post("/login", data = "<req>")]
pub async fn login(
    req: Json<LoginRequest>,
    mut db: Connection<UserDB>,
) -> Result<Json<String>, Status> {
    let row = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(&req.username)
        .fetch_one(&mut **db)
        .await
        .map_err(map_db_err)?;

    let user = User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password"),
    };

    if verify_password(&user.password_hash, &req.password) {
        let token = create_jwt(user.id).map_err(|_| Status::InternalServerError)?;
        return Ok(Json(token));
    }

    Err(Status::Unauthorized)
}

#[openapi]
#[post("/signup", data = "<req>")]
pub async fn signup(req: Json<LoginRequest>, mut db: Connection<UserDB>) -> Result<(), Status> {
    let _ = create_user(req, &mut db).await?;
    Ok(())
}

#[openapi]
#[post("/create-admin", data = "<req>")]
#[cfg(debug_assertions)]
pub async fn create_admin(
    req: Json<LoginRequest>,
    mut db: Connection<UserDB>,
) -> Result<(), Status> {
    // Create a new user
    let id = create_user(req, &mut db).await?;

    // Set the role to admin
    sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(Roles::Admin)
        .bind(id)
        .execute(&mut **db)
        .await
        .map_err(map_db_err)?;

    Ok(())
}

#[openapi]
#[get("/refresh")]
pub async fn refresh_token(user: AuthUser) -> Result<Json<String>, Status> {
    let token = create_jwt(user.0).map_err(|_| Status::InternalServerError)?;
    Ok(Json(token))
}
