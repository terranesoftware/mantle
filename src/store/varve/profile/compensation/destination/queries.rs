use framboid::account::profile::compensation::destination::{Destination as FramboidDestination, kind::DestinationKind as FramboidDestinationKind};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::compensation::destination::{DestinationRow, address::AddressRow, kind::Destination};

impl DestinationRow {
    queries! {
        foreign = compensation: i64;

        param = destination: &FramboidDestination;

        table = "destinations";

        names = ["kind"];

        binds = [Destination::from(destination)];

        recurse = async |pool, row: &Self| {
            match destination.kind() {
                FramboidDestinationKind::Default(address) | FramboidDestinationKind::Retirement(address) => AddressRow::insert(pool, row.id, address).await
            }
        };
    }
}