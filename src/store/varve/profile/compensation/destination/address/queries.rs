use framboid::account::profile::compensation::destination::address::{Address as FramboidAddress, kind::AddressKind as FramboidAddressKind};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::compensation::destination::address::{AddressRow, ach::AchRow, iban::IbanRow, kind::Address, pix::PixRow, upi::UpiRow};

impl AddressRow {
    queries! {
        foreign = destination: i64;

        param = address: &FramboidAddress;

        table = "destination.addresses";

        names = ["kind"];

        binds = [Address::from(address)];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            match address.kind() {
                FramboidAddressKind::Ach { account, kind, routing } => _ = AchRow::insert(pool, row.id, (account, *kind, routing)).await?,
                FramboidAddressKind::Iban(iban) => _ = IbanRow::insert(pool, row.id, iban).await?,
                FramboidAddressKind::Pix(key) => _ = PixRow::insert(pool, row.id, key).await?,
                FramboidAddressKind::Upi(vpa) => _ = UpiRow::insert(pool, row.id, vpa).await?
            }

            Ok(())
        };
    }
}