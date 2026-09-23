use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Destination(DestinationKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "destination_kind", rename_all = "lowercase")]
pub enum DestinationKind {
    Default,
    Retirement
}

impl From<&framboid::account::profile::compensation::destination::Destination> for Destination {
    fn from(value: &framboid::account::profile::compensation::destination::Destination) -> Self {
        match value.kind() {
            framboid::account::profile::compensation::destination::DestinationKind::Default(_) => Destination(DestinationKind::Default),
            framboid::account::profile::compensation::destination::DestinationKind::Retirement(_) => Destination(DestinationKind::Retirement)
        }
    }
}