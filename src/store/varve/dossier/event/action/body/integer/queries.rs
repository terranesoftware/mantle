use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::integer::IntegerRow;

impl IntegerRow {
    queries! {
        foreign = body;
        
        param = integer: i64;

        table = "integers";

        names = ["integer"];

        binds = [];
    }
}