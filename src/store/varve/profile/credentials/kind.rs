use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Credential(CredentialKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "credential_kind", rename_all = "lowercase")]
pub enum CredentialKind {
    Education
}

impl From<&framboid::account::profile::credentials::Credential> for Credential {
    fn from(value: &framboid::account::profile::credentials::Credential) -> Self {
        match value.kind() {
            framboid::account::profile::credentials::CredentialKind::Education { .. } => Credential(CredentialKind::Education)
        }
    }
}
