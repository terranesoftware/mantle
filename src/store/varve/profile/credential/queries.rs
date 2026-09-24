use framboid::account::profile::credentials::{Credential as FramboidCredential, CredentialKind as FramboidCredentialKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::credential::{CredentialRow, education::EducationRow, kind::Credential};

impl CredentialRow {
    queries! {
        foreign = profile;

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
