use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::boolean::BooleanRow;

impl BooleanRow {
    insert! {
        parameters = [body: i64, boolean: Body];

        setup = {
            // Decide how to error here later
            let boolean = match boolean.kind() {
                BodyKind::Boolean(bool) => bool,
                _ => unreachable!()
            };
        };

        row = BooleanRow;

        table = "booleans";

        columns = [1, 2];

        binds = [
            body,
            boolean
        ];
    }

    delete!("booleans");

    select!("booleans");

    update! {
        parameters = [body: Option<i64>, boolean: bool];

        table = "booleans";

        names = ["body", "boolean"];

        numbers = [2, 3];

        binds = [
            body,
            boolean
        ];
    }
}