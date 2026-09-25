use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

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