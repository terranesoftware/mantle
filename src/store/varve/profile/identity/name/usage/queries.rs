use framboid::account::profile::identity::name::usage::{Usage as FramboidUsage, UsageKind as FramboidUsageKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::identity::name::usage::{UsageRow, kind::Usage, prior::PriorRow};

impl UsageRow {
    queries! {
        foreign = name;

        param = usage: FramboidUsage;

        table = "usages";

        names = ["kind"];

        binds = [Usage::from(usage)];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            match usage.kind() {
                FramboidUsageKind::Prior(period) => _ = PriorRow::insert(pool, row.id, period).await?,
                _ => ()
            }

            Ok(())
        };
    }
}
