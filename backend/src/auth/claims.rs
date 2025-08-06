use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct Claims {
    pub sub: i32,
    pub exp: chrono::DateTime<chrono::Utc>,
}
