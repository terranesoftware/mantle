use framboid::keys::Key;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::{organization::key::OrganizationKey, store::organization::OrganizationRow};

impl OrganizationRow {
    queries! {
        param = organization: &OrganizationKey;

        table = "organizations";

        names = ["organization"];

        binds = [organization.key()];
    }
}