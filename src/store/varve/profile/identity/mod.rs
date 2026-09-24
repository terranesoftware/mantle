pub mod address;
pub mod name;
pub mod queries;

use sqlx::prelude::FromRow;
use time::Date;

#[derive(FromRow)]
pub struct IdentityRow {
    id: i64,
    profile: i64,

    birth: Date,
    emails: Vec<String>,
    phones: Vec<String>
}

impl IdentityRow {
    keys!(profile);

    /// Returns a copy of the contained birth date.
    pub fn birth(&self) -> Date {
        self.birth
    }

    /// Returns a reference to the contained emails.
    pub fn emails(&self) -> &[String] {
        &self.emails
    }

    /// Returns a reference to the contained phone numbers.
    pub fn phones(&self) -> &[String] {
        &self.phones
    }
}
