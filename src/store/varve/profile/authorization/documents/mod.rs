pub mod queries;

use sqlx::{postgres::types::PgRange, prelude::FromRow};
use time::Date;

#[derive(FromRow)]
pub struct DocumentRow {
    id: i64,
    authorization: i64,

    name: String,
    number: Option<String>,
    validity: Option<PgRange<Date>>
}

impl DocumentRow {
    keys!(authorization);

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the contained number if present.
    pub fn number(&self) -> Option<&str> {
        self.number.as_deref()
    }

    /// Returns a reference to the contained validity if present.
    pub fn validity(&self) -> Option<PgRange<Date>> {
        self.validity
    }
}