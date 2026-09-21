use sqlx::prelude::Type;

#[derive(Type)]
#[sqlx(type_name = "location")]
pub struct Location {
    city: String,
    country: String,
    county: String,
    state: String
}

impl From<&framboid::account::profile::location::Location> for Location {
    fn from(value: &framboid::account::profile::location::Location) -> Self {
        Self {
            city: value.city().to_string(),
            country: value.country().to_string(),
            county: value.county().to_string(),
            state: value.state().to_string()
        }
    }
}