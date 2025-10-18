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
mod routes;

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
                min_connections: config().blog.connections.min,
                max_connections: config().blog.connections.max,
                connect_timeout: config().blog.timeout.connect,
                idle_timeout: config().blog.timeout.idle,
                extensions: config().blog.extensions,
            },
        ))
        .merge((
            "databases.users",
            rocket_db_pools::Config {
                url: config().users.host.into(),
                min_connections: config().users.connections.min,
                max_connections: config().users.connections.max,
                connect_timeout: config().users.timeout.connect,
                idle_timeout: config().users.timeout.idle,
                extensions: config().users.extensions,
            },
        ));

    // Built server routes
    rocket::custom(rocket_config)
        .attach(BlogDB::init())
        .attach(UserDB::init())
        .mount(
            "/", // openapi_get_routes![list_test_entries, auth::login, auth::signup],
            routes::get_routes(),
        )
        .mount("/docs", make_swagger_ui(&ui()))
}
