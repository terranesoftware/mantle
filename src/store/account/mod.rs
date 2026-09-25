pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of an `AccountKey`.
#[derive(FromRow)]
pub struct AccountRow {
    id: i64,

    account: String
}

impl AccountRow {
    keys!();

    /// Returns a reference to the contained account.
    pub fn account(&self) -> &str {
        &self.account
    }
}