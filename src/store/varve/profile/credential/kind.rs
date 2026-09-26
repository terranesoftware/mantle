use framboid::account::profile::credentials::{Credential as FramboidCredential, kind::CredentialKind as FramboidCredentialKind};
use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Credential(CredentialKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "credential_kind", rename_all = "lowercase")]
pub enum CredentialKind {
    Education
}

impl From<&FramboidCredential> for Credential {
    fn from(value: &FramboidCredential) -> Self {
        match value.kind() {
            FramboidCredentialKind::Education { .. } => Credential(CredentialKind::Education)
        }
    }
}
