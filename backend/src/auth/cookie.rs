pub(super) struct Expires;

impl Expires {
    pub fn set(date: chrono::DateTime<chrono::Utc>) -> Result<time::OffsetDateTime, time::error::Error> {
        Ok(time::OffsetDateTime::from_unix_timestamp(date.timestamp())?)
    }

    pub fn get(date: time::OffsetDateTime) -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::<chrono::Utc>::from_timestamp(date.unix_timestamp(), 0)
    }
}
