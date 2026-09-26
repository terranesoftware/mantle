use framboid::account::profile::credentials::{Credential as FramboidCredential, kind::CredentialKind as FramboidCredentialKind};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::credential::{CredentialRow, education::EducationRow, kind::Credential};

impl CredentialRow {
    queries! {
        foreign = profile: i64;

        param = credential: &FramboidCredential;

        table = "credentials";

        names = ["kind"];

        binds = [Credential::from(credential)];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            match credential.kind() {
                FramboidCredentialKind::Education { degree, discipline, location, school, status } => _ = EducationRow::insert(pool, row.id, (degree, discipline, location, school, *status)).await?
            }

            Ok(())
        };
    }
}
