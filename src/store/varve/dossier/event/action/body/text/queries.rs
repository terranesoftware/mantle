use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::text::TextRow;

impl TextRow {
    queries! {
        foreign = body;
        
        param = text: &str;

        table = "texts";
        
        names = ["text"];

        binds = [text];
    }
}