use std::ops::Range;

use framboid::account::profile::authorization::document::Document;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, postgres::types::PgRange, query_as};

use crate::store::varve::profile::authorization::documents::DocumentRow;

impl DocumentRow {
    queries! {
        foreign = authorization;

        param = document: Document;

        table = "documents";

        names = ["name", "number", "validity"];

        binds = [
            document.name(),
            document.number(),
            document.validity().map(Range::from).map(PgRange::from)
        ];

        recurse = {};
    }
}