use framboid::account::profile::identity::Identity;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::identity::IdentityRow;

impl IdentityRow {
    queries! {
        foreign = profile;

        param = identity: &Identity;

        table = "identities";

        names = ["birth", "emails", "phones"];

        binds = [identity.birth(), identity.emails(), identity.phones()];

        recurse = {};
    }
}
