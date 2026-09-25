pub mod key;

use framboid::keys::{account::AccountKey, source::SourceKey};

use crate::organization::key::OrganizationKey;

/// An employing entity.
pub struct Organization {
    organization: OrganizationKey,
    sources: Vec<SourceKey>,
    mutations: Vec<AccountKey>
}

impl Organization {
    /// Creates an `Organization`, leaving the sources and mutations fields empty.
    pub fn new(organization: OrganizationKey) -> Self {
        Self {
            organization,
            sources: Vec::new(),
            mutations: Vec::new()
        }
    }

    /// Returns a reference to the contained organization.
    pub fn organization(&self) -> &OrganizationKey {
        &self.organization
    }

    /// Returns a reference to the contained sources.
    pub fn sources(&self) -> &[SourceKey] {
        &self.sources
    }

    /// Returns a reference to the contained mutations.
    pub fn mutations(&self) -> &[AccountKey] {
        &self.mutations
    }
}