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

impl From<&framboid::account::profile::engagement::arrangement::Arrangement> for Arrangement {
    fn from(value: &framboid::account::profile::engagement::arrangement::Arrangement) -> Self {
        match value.kind() {
            framboid::account::profile::engagement::arrangement::ArrangementKind::Hybrid => Arrangement(ArrangementKind::Hybrid),
            framboid::account::profile::engagement::arrangement::ArrangementKind::OnSite => Arrangement(ArrangementKind::OnSite),
            framboid::account::profile::engagement::arrangement::ArrangementKind::Remote => Arrangement(ArrangementKind::Remote)
        }
    }
}
