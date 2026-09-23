pub mod kind;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::compensation::destination::address::kind::Address;

#[derive(FromRow)]
pub struct AddressRow {
    id: i64,
    destination: i64,

    kind: Address
}