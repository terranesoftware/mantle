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

impl From<&framboid::account::profile::compensation::destination::address::account::Account> for Account {
    fn from(value: &framboid::account::profile::compensation::destination::address::account::Account) -> Self {
        match value.kind() {
            framboid::account::profile::compensation::destination::address::account::AccountKind::Checking => Account(AccountKind::Checking),
            framboid::account::profile::compensation::destination::address::account::AccountKind::Savings => Account(AccountKind::Savings)
        }
    }
}
