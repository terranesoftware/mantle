pub mod handler;

use axum::{extract::FromRequestParts, http::StatusCode};
use framboid::keys::account::AccountKey;

use crate::organization::key::OrganizationKey;

#[derive(Clone)]
pub struct Principal {
    account: AccountKey,
    organization: Option<OrganizationKey>
}

impl Principal {
    pub fn account(&self) -> &AccountKey {
        &self.account
    }

    pub fn organization(&self) -> Option<&OrganizationKey> {
        self.organization.as_ref()
    }
}

impl<S> FromRequestParts<S> for Principal
where
    S: Send + Sync
{
    type Rejection = StatusCode;
    
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection>
    {
        parts.extensions
            .get()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}