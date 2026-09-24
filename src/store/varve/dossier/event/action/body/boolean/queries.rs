use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::boolean::BooleanRow;

impl BooleanRow {
    queries! {
        foreign = body;

        param = boolean: bool;

        table = "booleans";

        names = ["boolean"];

        binds = [boolean];
    }
}