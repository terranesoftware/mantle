use crate::store::varve::profile::{ProfileRow, authorization::AuthorizationRow, compensation::CompensationRow, credential::CredentialRow, engagement::EngagementRow, identity::IdentityRow};
use framboid::account::profile::Profile;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl ProfileRow {
    queries! {
        foreign = varve;

        param = profile: &Profile;

        table = "profiles";

        names = [];

        binds = [];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            AuthorizationRow::insert(pool, row.id, profile.authorization()).await?;
            CompensationRow::insert(pool, row.id, profile.compensation()).await?;
            for credential in profile.credentials() {
                CredentialRow::insert(pool, row.id, credential).await?;
            }
            for engagement in profile.engagements() {
                EngagementRow::insert(pool, row.id, engagement).await?;
            }
            IdentityRow::insert(pool, row.id, profile.identity()).await?;

            Ok(())
        };
    }
}