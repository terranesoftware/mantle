use framboid::account::profile::identity::name::Name;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::identity::name::{NameRow, identifiers::Identifiers, usage::UsageRow};

impl NameRow {
    queries! {
        foreign = identity;

        param = name: &Name;

        table = "names";

        names = ["latin", "native"];

        binds = [name.latin().map(Identifiers::from), Identifiers::from(name.native())];

        recurse = async |pool, row: &Self| {
            UsageRow::insert(pool, row.id, name.usage()).await
        };
    }
}
