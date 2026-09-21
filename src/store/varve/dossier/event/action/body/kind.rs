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

impl From<&framboid::addressing::body::Body> for Body {
    fn from(value: &framboid::addressing::body::Body) -> Self {
        match value.kind() {
            framboid::addressing::body::BodyKind::Boolean(_) => Body(BodyKind::Boolean),
            framboid::addressing::body::BodyKind::Handle(_) => Body(BodyKind::Handle),
            framboid::addressing::body::BodyKind::Integer(_) => Body(BodyKind::Handle),
            framboid::addressing::body::BodyKind::List(_) => Body(BodyKind::List),
            framboid::addressing::body::BodyKind::Map(_) => Body(BodyKind::Map),
            framboid::addressing::body::BodyKind::Text(_) => Body(BodyKind::Text)
        }
    }
}