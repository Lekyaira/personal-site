use figment::{Figment, providers::Env};
use rocket::{get, serde::json::Json};
use rocket_db_pools::{Connection, Database};
use rocket_okapi::{
    okapi::{schemars, schemars::JsonSchema},
    openapi, openapi_get_routes,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

// Make sqlx available at crate root for derive macros
extern crate rocket_db_pools;
use rocket_db_pools::sqlx::{self, Row};

// Server configuration
#[derive(Clone, PartialEq, Deserialize)]
struct ServerConfig {
    host: String,
    secret: String,
}

#[derive(Database)]
#[database("blog")]
struct BlogDB(rocket_db_pools::sqlx::PgPool);

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

fn ui() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".into(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    // Import config
    dotenvy::dotenv().ok(); // Set env vars from .env file
    let config: ServerConfig = Figment::new().merge(Env::raw()).extract().unwrap();

    // Insert database url into Rocket
    let rocket_config = rocket::Config::figment().merge((
        "databases.blog",
        rocket_db_pools::Config {
            url: config.host.into(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
            extensions: None,
        },
    ));

    // Built server routes
    rocket::custom(rocket_config)
        .attach(BlogDB::init())
        .mount("/", openapi_get_routes![list_test_entries])
        .mount("/docs", make_swagger_ui(&ui()))
}
