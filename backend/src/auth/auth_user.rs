use super::claims::Claims;
use crate::config::config;
use jsonwebtoken::{DecodingKey, Validation, decode};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use rocket_okapi::{
    r#gen::OpenApiGenerator,
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
};

/// Authorizes a user for endpoint request
///
/// # Example
/// ```rust
/// // Protected Rocket endpoint
/// #[get("/me")]
/// async fn me(user: AuthUser) -> Json<i32> {
///     Json(user.0) // user id
/// }
/// ```
pub struct AuthUser(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(str::to_string);

        if let Some(token) = token {
            let result = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(config().secret.as_ref()),
                &Validation::default(),
            );

            if let Ok(data) = result {
                return Outcome::Success(AuthUser(data.claims.sub));
            }
        }

        Outcome::Error((Status::Unauthorized, Status::Unauthorized))
    }
}

impl<'a> OpenApiFromRequest<'a> for AuthUser {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Bearer token to access, token is: `mytoken`.".to_owned(),
            ),
            // Setup data requirements.
            // In this case the header `Authorization: mytoken` needs to be set.
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(), // `basic`, `digest`, ...
                // Just gives use a hint to the format used
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
