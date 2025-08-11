use super::auth_user::AuthUser;
use super::db::create_user;
use super::login_request::LoginRequest;
use super::password::verify_password;
use super::roles::Roles;
use super::token::{create_jwt, get_claims};
use super::user::User;
use super::cookie::{Expires, get_user_claims};
use crate::db::UserDB;
use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    serde::json::Json,
};
use rocket_db_pools::{Connection, sqlx::{Row, Postgres}};
use rocket_okapi::openapi;
use chrono::{Utc, Duration};

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

/// Logs in a user given username and password. JWT token saved in browser cookies.
#[openapi]
#[post("/login", data = "<req>")]
pub async fn login(
    req: Json<LoginRequest>,
    jar: &CookieJar<'_>,
    mut db: Connection<UserDB>,
) -> Result<Json<User>, Status> {
    let row = sqlx::query("SELECT username, id, password, fullname, callby FROM users WHERE username = $1")
        .bind(&req.username)
        .fetch_one(&mut **db)
        .await
        .map_err(map_db_err)?;

    let user = User {
        username: row.get("username"),
        fullname: row.get("fullname"),
        callby: row.get("callby"),
    };
    let id = row.get("id");
    let password_hash = row.get("password");
    verify_password(password_hash, &req.password)?;

    // Password ok, continue login
    let expiration = Utc::now() + Duration::weeks(1);
    let token = create_jwt(id, expiration).map_err(|_| Status::InternalServerError)?;
    // Save the token as a cookie
    // Set as HttpOnly, Secure, SameSite <- Security features, change with caution!
    let mut cookie = Cookie::new("token", token);
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");
    if req.stay_logged_in {
        cookie.set_expires(Expires::set(expiration).map_err(|_| Status::InternalServerError)?);
    } else {
        cookie.set_expires(None); // Expires with session
    }
    jar.add(cookie);

    Ok(Json(user))
}

/// Logs out current user by removing the token cookie
#[openapi]
#[get("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Result<Json<String>, Status> {
    jar.remove("token");
    Ok(Json("ok".into()))
}

/// Adds a new user to the database
#[openapi]
#[post("/signup", data = "<req>")]
pub async fn signup(req: Json<LoginRequest>, mut db: Connection<UserDB>) -> Result<(), Status> {
    let _ = create_user(req, &mut db).await?;
    Ok(())
}

/// Creates an admin user account. Only compiled with `debug_assertions`. VERIFY NO ENDPOINT IN
/// RELEASE CODE!!!
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

/// Refreshes logged in user expiration (if any) and returns `User` data. If token does not exist,
/// just returns `Status::Ok`.
#[openapi]
#[get("/me")]
pub async fn me(
    jar: &CookieJar<'_>,
    mut db: Connection<UserDB>,
) -> Result<Json<User>, Status> {
    // Get the user token from the cookie, if it exists
    let (claims, expires) = get_user_claims(&jar)?;

    // Get the user data
    let row = sqlx::query("SELECT username, fullname, callby FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_one(&mut **db)
        .await
        .map_err(map_db_err)?;

    let userData = User {
        username: row.get("username"),
        fullname: row.get("fullname"),
        callby: row.get("callby"),
    };

    // Regenerate the token
    let expiration = Utc::now() + Duration::weeks(1);
    let token = create_jwt(claims.sub, expiration).map_err(|_| Status::InternalServerError)?;

    // Save the cookie with the new information
    let mut cookie = Cookie::new("token", token);
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_path("/");
    if expires.is_some() {
        cookie.set_expires(Expires::set(expiration).map_err(|_| Status::InternalServerError)?);
    } else {
        cookie.set_expires(None); // Expires with session
    }
    jar.add(cookie);

    Ok(Json(userData))
}

#[openapi]
#[get("/links")]
pub async fn links(
    jar: &CookieJar<'_>,
    mut db: Connection<UserDB>,
) -> Result<Json<Vec<super::link::Link>>, Status> {
    use super::link::Link;

    // Get the user claims. It's fine if the cookie doesn't exist, but not if it's invalid.
    let rows = match get_user_claims(&jar) {
        Err(_) => { // Guest
            sqlx::query("SELECT * FROM get_guest_links()")
                .fetch_all(&mut **db)
                .await
                .map_err(map_db_err)?
                .into_iter()
        },
        Ok(claims) => { // Authenticated user
            sqlx::query("SELECT * FROM get_user_links($1)")
                .bind(claims.0.sub)
                .fetch_all(&mut **db)
                .await
                .map_err(map_db_err)?
                .into_iter()
        },
    };
    let links: Vec<Link> = rows.map(|r| Link { name: r.get("name"), to: r.get("href") }).collect();

    Ok(Json(links))
}

#[openapi]
#[get("/admin")]
pub async fn admin(
    user: AuthUser,
    mut db: Connection<UserDB>,
) -> Result<Json<bool>, Status> {
    let is_admin: bool = sqlx::query_scalar::<Postgres, bool>("SELECT is_admin($1)")
        .bind(user.0)
        .fetch_one(&mut **db)
        .await
        .map_err(map_db_err)?;
    Ok(Json(is_admin))
}
