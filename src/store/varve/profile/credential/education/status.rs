use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Status(StatusKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum StatusKind {
    Completed,
    Enrolled,
    DroppedOut,
    Transferred,
    Withdrawn
}

impl From<&framboid::account::profile::credentials::status::Status> for Status {
    fn from(value: &framboid::account::profile::credentials::status::Status) -> Self {
        match value.kind() {
            framboid::account::profile::credentials::status::StatusKind::Completed => Status(StatusKind::Completed),
            framboid::account::profile::credentials::status::StatusKind::Enrolled => Status(StatusKind::Enrolled),
            framboid::account::profile::credentials::status::StatusKind::DroppedOut => Status(StatusKind::DroppedOut),
            framboid::account::profile::credentials::status::StatusKind::Transferred => Status(StatusKind::Transferred),
            framboid::account::profile::credentials::status::StatusKind::Withdrawn => Status(StatusKind::Withdrawn)
        }
    }
}
