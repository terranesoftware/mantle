use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::handle::HandleRow;

impl HandleRow {
    insert! {
        parameters = [body: i64, handle: Body];

        setup = {
            // Decide how to error here later
            let handle = match handle.kind() {
                BodyKind::Handle(str) => str,
                _ => unreachable!()
            };
        };

        row = HandleRow;

        table = "handles";

        columns = [1, 2];

        binds = [body, handle];
    }

    delete!("handles");

    select!("handles");

    update! {
        parameters = [body: Option<i64>, handle: &str];

        table = "handles";

        names = ["body", "handle"];

        numbers = [2, 3];

        binds = [body, handle];
    }
}