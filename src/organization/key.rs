use framboid::keys::Key;

/// An organization identifier.
#[derive(Clone)]
pub struct OrganizationKey(String);

impl Key for OrganizationKey {
    /// Creates an `OrganizationKey`.
    fn new(key: String) -> Self {
        Self(key)
    }

    /// Returns a reference to the contained key.
    fn key(&self) -> &str {
        &self.0
    }
}