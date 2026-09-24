use framboid::account::profile::identity::Identity;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::identity::{IdentityRow, address::AddressRow, name::NameRow};

impl IdentityRow {
    queries! {
        foreign = profile;

        param = identity: &Identity;

        table = "identities";

        names = ["birth", "emails", "phones"];

        binds = [identity.birth(), identity.emails(), identity.phones()];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            AddressRow::insert(pool, row.id, identity.address()).await?;

            for name in identity.names() {
                NameRow::insert(pool, row.id, name).await?;
            }

            Ok(())
        };
    }
}
