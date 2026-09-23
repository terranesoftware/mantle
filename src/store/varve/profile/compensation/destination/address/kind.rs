use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Address(AddressKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "address_kind", rename_all = "lowercase")]
pub enum AddressKind {
    Ach,
    Iban,
    Pix,
    Upi
}

impl From<&framboid::account::profile::compensation::destination::address::Address> for Address {
    fn from(value: &framboid::account::profile::compensation::destination::address::Address) -> Self {
        match value.kind() {
            framboid::account::profile::compensation::destination::address::AddressKind::Ach { .. } => Address(AddressKind::Ach),
            framboid::account::profile::compensation::destination::address::AddressKind::Iban(_) => Address(AddressKind::Iban),
            framboid::account::profile::compensation::destination::address::AddressKind::Pix(_) => Address(AddressKind::Pix),
            framboid::account::profile::compensation::destination::address::AddressKind::Upi(_) => Address(AddressKind::Upi)
        }
    }
}