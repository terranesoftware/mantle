use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::dossier::event::action::body::handle::HandleRow;

impl HandleRow {
    queries! {
        foreign = body;

        param = handle: &str;

        table = "handles";

        names = ["handle"];

        binds = [handle];
    }
}