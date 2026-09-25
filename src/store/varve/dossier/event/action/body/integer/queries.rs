use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::dossier::event::action::body::integer::IntegerRow;

impl IntegerRow {
    queries! {
        foreign = body;
        
        param = integer: i64;

        table = "integers";

        names = ["integer"];

        binds = [integer];
    }
}