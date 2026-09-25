pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct IbanRow {
    id: i64,
    address: i64,

    iban: String
}

impl IbanRow {
    keys!(address: i64);

    /// Returns a reference to the contained IBAN.
    pub fn iban(&self) -> &str {
        &self.iban
    }
}
