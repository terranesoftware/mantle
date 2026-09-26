use framboid::account::profile::compensation::destination::address::account::{Account as FramboidAccount, kind::AccountKind as FramboidAccountKind};
use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Account(AccountKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "account_kind", rename_all = "lowercase")]
pub enum AccountKind {
    Checking,
    Savings
}

impl From<FramboidAccount> for Account {
    fn from(value: FramboidAccount) -> Self {
        match value.kind() {
            FramboidAccountKind::Checking => Account(AccountKind::Checking),
            FramboidAccountKind::Savings => Account(AccountKind::Savings)
        }
    }
}
