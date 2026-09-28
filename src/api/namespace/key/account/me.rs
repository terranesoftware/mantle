use framboid::keys::account::AccountKey;

use crate::api::utilities::{bitcode::Bitcode, principal::Principal};

pub async fn me(principal: Principal) -> Bitcode<AccountKey> {
    Bitcode(principal.account().clone())
}