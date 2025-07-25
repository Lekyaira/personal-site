use figment::{Figment, providers::Env};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::sync::Mutex;

/// Blog database configuration data
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct BlogConfig {
    pub host: String,
    // TODO: Add other server config variables
    // TODO: Add sensible defaults
}

/// User database configuration data
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct UsersConfig {
    pub host: String,
    // TODO: Add other server config variables
    // TODO: Add sensible defaults
}

/// Server configuration data
#[derive(Clone, PartialEq, Deserialize)]
pub struct ServerConfig {
    pub blog: BlogConfig,
    pub users: UsersConfig,
    pub secret: String,
}

/// Returns the server configuration data.
/// Pulls ServerConfig from .env file and environment variables on initialization.
pub fn config() -> ServerConfig {
    static INSTANCE: OnceCell<Mutex<ServerConfig>> = OnceCell::new();
    let config = INSTANCE
        .get_or_init(|| {
            // Initialize configuration
            // Import config
            dotenvy::dotenv().ok(); // Set env vars from .env file
            let config: ServerConfig = Figment::new()
                .merge(Env::raw().split("_"))
                .extract()
                .unwrap();
            Mutex::new(config)
        })
        .lock()
        .unwrap();
    config.clone()
}
