use sqlx::prelude::Type;

#[derive(Type)]
#[sqlx(type_name = "identifiers")]
pub struct Identifiers {
    primary: String,
    secondary: Option<String>
}

impl From<&framboid::account::profile::identity::name::identifiers::Identifiers> for Identifiers {
    fn from(value: &framboid::account::profile::identity::name::identifiers::Identifiers) -> Self {
        Self {
            primary: value.primary().to_string(),
            secondary: value.secondary().map(str::to_string)
        }
    }
}
