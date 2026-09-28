use framboid::keys::account::AccountKey;

use crate::api::utilities::{authentication::principal::Principal, bitcode::Bitcode};

pub async fn me(principal: Principal) -> Bitcode<AccountKey> {
    Bitcode(principal.account().clone())
}