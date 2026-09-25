use framboid::keys::{Key, account::AccountKey};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::account::AccountRow;

impl AccountRow {
    queries! {
        foreign = organization: Option<i64>;
        
        param = account: &AccountKey;

        table = "accounts";

        names = ["account"];

        binds = [account.key()];
    }
}