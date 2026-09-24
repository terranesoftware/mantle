use framboid::addressing::body::{Body as FramboidBody, BodyKind as FramboidBodyKind};
use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Body(BodyKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "body_kind", rename_all = "lowercase")]
pub enum BodyKind {
    Boolean,
    Handle,
    Integer,
    List,
    Map,
    Text
}

impl From<&FramboidBody> for Body {
    fn from(value: &FramboidBody) -> Self {
        match value.kind() {
            FramboidBodyKind::Boolean(_) => Body(BodyKind::Boolean),
            FramboidBodyKind::Handle(_) => Body(BodyKind::Handle),
            FramboidBodyKind::Integer(_) => Body(BodyKind::Integer),
            FramboidBodyKind::List(_) => Body(BodyKind::List),
            FramboidBodyKind::Map(_) => Body(BodyKind::Map),
            FramboidBodyKind::Text(_) => Body(BodyKind::Text)
        }
    }
}