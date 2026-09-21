use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::list::ListRow;

impl ListRow {
    insert! {
        parameters = [body: i64, list: &Body];

        setup = {
            let list = match list.kind() {
                BodyKind::List(list) => list,
                _ => unreachable!()
            };
        };

        row = ListRow;

        table = "lists";

        columns = [1];

        binds = [body];

        // Complete this
        recurse = {};
    }

    delete!("lists");

    select!("lists");

    update! {
        parameters = [body: Option<i64>];

        table = "lists";

        names = ["body"];

        numbers = [2];

        binds = [body];
    }
}