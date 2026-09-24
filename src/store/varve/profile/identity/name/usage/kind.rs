use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Usage(UsageKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "usage_kind", rename_all = "lowercase")]
pub enum UsageKind {
    Legal,
    Prior,
    Used
}

impl From<&framboid::account::profile::identity::name::usage::Usage> for Usage {
    fn from(value: &framboid::account::profile::identity::name::usage::Usage) -> Self {
        match value.kind() {
            framboid::account::profile::identity::name::usage::UsageKind::Legal => Usage(UsageKind::Legal),
            framboid::account::profile::identity::name::usage::UsageKind::Prior(_) => Usage(UsageKind::Prior),
            framboid::account::profile::identity::name::usage::UsageKind::Used => Usage(UsageKind::Used)
        }
    }
}
