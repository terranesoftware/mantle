use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::dossier::event::action::body::boolean::BooleanRow;

impl BooleanRow {
    queries! {
        foreign = body: i64;

        param = boolean: bool;

        table = "booleans";

        names = ["boolean"];

        binds = [boolean];
    }
}