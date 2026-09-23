use framboid::account::profile::credentials::Credential;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::credentials::{CredentialRow, kind};

impl CredentialRow {
    queries! {
        foreign = profile;

        param = credential: &Credential;

        table = "credentials";

        names = ["kind"];

        binds = [kind::Credential::from(credential)];

        recurse = {};
    }
}
