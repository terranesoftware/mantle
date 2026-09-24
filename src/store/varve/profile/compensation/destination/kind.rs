use framboid::account::profile::compensation::destination::{Destination as FramboidDestination, DestinationKind as FramboidDestinationKind};
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

impl From<&FramboidDestination> for Destination {
    fn from(value: &FramboidDestination) -> Self {
        match value.kind() {
            FramboidDestinationKind::Default(_) => Destination(DestinationKind::Default),
            FramboidDestinationKind::Retirement(_) => Destination(DestinationKind::Retirement)
        }
    }
}