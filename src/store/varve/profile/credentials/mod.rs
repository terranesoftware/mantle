pub mod education;
pub mod kind;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::credentials::kind::Credential;

#[derive(FromRow)]
pub struct CredentialRow {
    id: i64,
    profile: i64,

    kind: Credential
}

impl CredentialRow {
    keys!(profile);

    /// Returns a copy of the contained `Credential`.
    pub fn kind(&self) -> Credential {
        self.kind
    }
}
