pub mod account;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::compensation::destination::address::ach::account::Account;

#[derive(FromRow)]
pub struct AchRow {
    id: i64,
    address: i64,

    account: String,
    kind: Account,
    routing: String
}

impl AchRow {
    keys!(address: i64);

    /// Returns a reference to the contained account.
    pub fn account(&self) -> &str {
        &self.account
    }

    /// Returns a copy of the contained `Account`.
    pub fn kind(&self) -> Account {
        self.kind
    }

    /// Returns a reference to the contained routing number.
    pub fn routing(&self) -> &str {
        &self.routing
    }
}
