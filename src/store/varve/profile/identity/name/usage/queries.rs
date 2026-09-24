use framboid::account::profile::identity::name::usage::Usage;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::identity::name::usage::{UsageRow, kind};

impl UsageRow {
    queries! {
        foreign = name;

        param = usage: Usage;

        table = "usages";

        names = ["kind"];

        binds = [kind::Usage::from(&usage)];

        recurse = {};
    }
}
