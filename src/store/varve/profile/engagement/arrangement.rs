use framboid::account::profile::engagement::arrangement::{Arrangement as FramboidArrangement, kind::ArrangementKind as FramboidArrangementKind};
use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Arrangement(ArrangementKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "arrangement", rename_all = "snake_case")]
pub enum ArrangementKind {
    Hybrid,
    OnSite,
    Remote
}

impl From<FramboidArrangement> for Arrangement {
    fn from(value: FramboidArrangement) -> Self {
        match value.kind() {
            FramboidArrangementKind::Hybrid => Arrangement(ArrangementKind::Hybrid),
            FramboidArrangementKind::OnSite => Arrangement(ArrangementKind::OnSite),
            FramboidArrangementKind::Remote => Arrangement(ArrangementKind::Remote)
        }
    }
}
