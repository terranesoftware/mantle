use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

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