pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct UpiRow {
    id: i64,
    address: i64,

    vpa: String
}

impl UpiRow {
    keys!(address: i64);

    /// Returns a reference to the contained VPA.
    pub fn vpa(&self) -> &str {
        &self.vpa
    }
}
