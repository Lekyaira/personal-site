use serde::{Deserialize, Serialize};

/// Represents JWT claims
#[derive(Serialize, Deserialize)]
pub(super) struct Claims {
    /// User ID
    pub sub: i32,
    /// Expiration date of the token
    pub exp: chrono::DateTime<chrono::Utc>,
}
