use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::text::TextRow;

impl TextRow {
    queries! {
        foreign = [body, i64, "body", body];
        
        parameters = [text: &Body];

        table = "texts";
        
        names = ["body", "text"];

        binds = [
            body,
            match text.kind() {
                BodyKind::Text(text) => text,
                _ => unreachable!()
            }
        ];
    }
}