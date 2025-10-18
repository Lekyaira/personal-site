use crate::auth;
use rocket::{get, serde::json::Json};
use rocket_okapi::{
    okapi::{schemars, schemars::JsonSchema},
    openapi, openapi_get_routes,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};

pub fn get_routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        test_admin,
        auth::login,
        auth::logout,
        auth::signup,
        auth::create_admin,
        auth::me,
        auth::links,
        auth::admin,
    ]
}

#[openapi]
#[get("/test-admin")]
async fn test_admin(user: auth::AuthUser) -> Json<String> {
    format!("You are logged in as admin. User id: {}", user.0).into()
}
