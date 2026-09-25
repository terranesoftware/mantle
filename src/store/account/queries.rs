use framboid::keys::{Key, account::AccountKey};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::account::AccountRow;

impl AccountRow {
    queries! {
        param = account: &AccountKey;

        table = "accounts";

        names = ["account"];

        binds = [account.key()];
    }
}