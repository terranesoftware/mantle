use framboid::account::profile::identity::name::Name;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

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
