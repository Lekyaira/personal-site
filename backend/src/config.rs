use figment::{Figment, providers::Env};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::sync::Mutex;
use crate::rocket::yansi::Paint;

/// Database configuration data
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct DBConfig {
    pub host: String,
    #[serde(default)]
    pub connections: Connections,
    #[serde(default)]
    pub timeout: Timeouts,
    #[serde(default = "extensions")]
    pub extensions: Option<Vec<String>>,
}

/// Database connection bounds data
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Connections {
    #[serde(default = "min_connections")]
    pub min: Option<u32>,
    #[serde(default = "max_connections")]
    pub max: usize,
}

impl Default for Connections {
    fn default() -> Self {
        Self {
            min: min_connections(),
            max: max_connections(),
        }
    }
}

/// Database connection timeout data
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Timeouts {
    #[serde(default = "connect_timeout")]
    pub connect: u64,
    #[serde(default = "idle_timeout")]
    pub idle: Option<u64>,
}

impl Default for Timeouts {
    fn default() -> Self {
        Self {
            connect: connect_timeout(),
            idle: idle_timeout(),
        }
    }
}

// DBConfig defaults
fn min_connections() -> Option<u32> { None }
fn max_connections() -> usize { 1024 }
fn connect_timeout() -> u64 { 3 }
fn idle_timeout() -> Option<u64> { None }
fn extensions() -> Option<Vec<String>> { None }

/// Server configuration data
#[derive(Clone, PartialEq, Deserialize)]
pub struct ServerConfig {
    pub blog: DBConfig,
    pub users: DBConfig,
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
