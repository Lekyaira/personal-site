use serde::{Deserialize, Serialize};
use chrono::{DateTime, TimeZone, Utc};

/// Represents JWT claims
#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Claims {
    /// User ID
    pub sub: i32,
    /// Expiration date of the token
    pub exp: usize,
}

impl Claims {
    pub fn expires(&self) -> DateTime<Utc> {
        Utc.timestamp(self.exp as i64, 0)
    }
}
