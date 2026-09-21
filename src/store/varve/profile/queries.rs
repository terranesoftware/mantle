use crate::store::varve::profile::ProfileRow;
use sqlx::{Error, PgPool, query_as};

impl ProfileRow {
    insert! {
        parameters = [varve: i64];

        row = ProfileRow;

        table = "profiles";

        columns = [1];

        binds = [varve];

        // Complete this
        recurse = {};
    }

    delete!("profiles");

    select!("profiles");

    update! {
        parameters = [varve: Option<i64>];

        table = "profiles";

        names = ["varve"];

        numbers = [2];

        binds = [varve];
    }
}