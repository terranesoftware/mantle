use framboid::account::profile::compensation::destination::address::{Address as FramboidAddress, AddressKind as FramboidAddressKind};
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

impl From<&FramboidAddress> for Address {
    fn from(value: &FramboidAddress) -> Self {
        match value.kind() {
            FramboidAddressKind::Ach { .. } => Address(AddressKind::Ach),
            FramboidAddressKind::Iban(_) => Address(AddressKind::Iban),
            FramboidAddressKind::Pix(_) => Address(AddressKind::Pix),
            FramboidAddressKind::Upi(_) => Address(AddressKind::Upi)
        }
    }
}