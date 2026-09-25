use crate::store::varve::profile::authorization::{AuthorizationRow, documents::DocumentRow};
use framboid::account::profile::authorization::Authorization;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

impl AuthorizationRow {
    queries! {
        foreign = profile;

        param = authorization: &Authorization;

        table = "authorizations";

        names = [];

        binds = [];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for document in authorization.documents() {
                DocumentRow::insert(pool, row.id, document).await?;
            }

            Ok(())
        };
    }
}