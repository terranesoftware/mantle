use framboid::keys::Key;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::{organization::key::OrganizationKey, store::organization::OrganizationRow};

impl OrganizationRow {
    queries! {
        param = organization: &OrganizationKey;

        table = "organizations";

        names = ["organization"];

        binds = [organization.key()];
    }
}