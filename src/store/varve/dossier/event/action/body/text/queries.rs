use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::text::TextRow;

impl TextRow {
    insert! {
        parameters = [body: i64, text: Body];

        setup = {
            let text = match text.kind() {
                BodyKind::Text(text) => text,
                _ => unreachable!()
            };
        };

        row = TextRow;

        table = "texts";

        columns = [1, 2];

        binds = [body, text];
    }

    delete!("texts");

    select!("texts");

    update! {
        parameters = [body: Option<i64>, text: &str];

        table = "texts";

        names = ["body", "text"];

        numbers = [2, 3];

        binds = [body, text];
    }
}