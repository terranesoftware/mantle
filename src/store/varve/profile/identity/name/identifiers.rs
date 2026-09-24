use framboid::account::profile::identity::name::identifiers::Identifiers as FramboidIdentifiers;
use sqlx::prelude::Type;

#[derive(Type)]
#[sqlx(type_name = "identifiers")]
pub struct Identifiers {
    primary: String,
    secondary: Option<String>
}

impl From<&FramboidIdentifiers> for Identifiers {
    fn from(value: &FramboidIdentifiers) -> Self {
        Self {
            primary: value.primary().to_string(),
            secondary: value.secondary().map(str::to_string)
        }
    }
}
