use framboid::account::profile::compensation::destination::{Destination as FramboidDestination, DestinationKind as FramboidDestinationKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::{DestinationRow, address::AddressRow, kind::Destination};

impl DestinationRow {
    queries! {
        foreign = compensation;

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