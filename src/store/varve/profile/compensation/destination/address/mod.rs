pub mod ach;
pub mod iban;
pub mod kind;
pub mod pix;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::compensation::destination::address::kind::Address;

#[derive(FromRow)]
pub struct AddressRow {
    id: i64,
    destination: i64,

    kind: Address
}

impl AddressRow {
    keys!(destination);

    /// Returns a copy of the contained `Address`.
    pub fn kind(&self) -> Address {
        self.kind
    }
}