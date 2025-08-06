/// Helper struct to convert `chrono` Utc time into `time` OffsetDateTime for use in Rocket
/// Cookie.expires()
pub(super) struct Expires;

impl Expires {
    /// Converts `chrono` Utc date into Rocket Cookie.expires() `time` date
    /// # Arguments
    /// - `date`: `chrono::DateTime<Utc>`
    /// # Returns
    /// - `Result<time::OffsetDateTime, time::error::Error>`
    pub fn set(date: chrono::DateTime<chrono::Utc>) -> Result<time::OffsetDateTime, time::error::Error> {
        Ok(time::OffsetDateTime::from_unix_timestamp(date.timestamp())?)
    }

    /// Converts Rocket Cookie.expires() `time` date into `chrono` Utc date
    /// # Arguments
    /// - `date`: `time::OffsetDateTime`
    /// # Returns
    /// - `Option<chrono::DateTime<Utc>>` - Returns date time in UTC in millisecond precision
    pub fn get(date: time::OffsetDateTime) -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::<chrono::Utc>::from_timestamp(date.unix_timestamp(), 0)
    }
}
