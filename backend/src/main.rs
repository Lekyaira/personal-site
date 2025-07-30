use rocket::{get, serde::json::Json};
use rocket_db_pools::{Connection, Database};
use rocket_okapi::{
    okapi::{schemars, schemars::JsonSchema},
    openapi, openapi_get_routes,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};
use serde::Serialize;

#[macro_use]
extern crate rocket;

// Make sqlx available at crate root for derive macros
extern crate rocket_db_pools;
use rocket_db_pools::sqlx::{self, Row};

mod config;
use config::config;
mod auth;
use auth::AuthUser;
mod db;
use db::{BlogDB, UserDB};

#[derive(Debug, Serialize, JsonSchema)]
struct Test {
    id: i32,
    title: String,
    body: String,
}

#[openapi]
#[get("/test")]
async fn list_test_entries(mut db: Connection<BlogDB>) -> Json<Vec<Test>> {
    let rows = sqlx::query("SELECT id, title, body FROM test")
        .fetch_all(&mut **db)
        .await
        .expect("Failed to fetch test entries");

    let entries: Vec<Test> = rows
        .into_iter()
        .map(|row| Test {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
        })
        .collect();

    Json(entries)
}

#[openapi]
#[get("/test-admin")]
async fn test_admin(user: AuthUser) -> Json<String> {
    format!("You are logged in as admin. User id: {}", user.0).into()
}

fn ui() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".into(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    // Insert database url into Rocket
    // TODO: Pull the rest of the Rocket config values from server config
    let rocket_config = rocket::Config::figment()
        .merge((
            "databases.blog",
            rocket_db_pools::Config {
                url: config().blog.host.into(),
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
                extensions: None,
            },
        ))
        .merge((
            "databases.users",
            rocket_db_pools::Config {
                url: config().users.host.into(),
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
                extensions: None,
            },
        ));

    let routes = openapi_get_routes![
        list_test_entries,
        test_admin,
        auth::login,
        auth::signup,
        auth::create_admin,
        auth::refresh_token,
    ];

    // Built server routes
    rocket::custom(rocket_config)
        .attach(BlogDB::init())
        .attach(UserDB::init())
        .mount(
            "/", // openapi_get_routes![list_test_entries, auth::login, auth::signup],
            routes,
        )
        .mount("/docs", make_swagger_ui(&ui()))
}
