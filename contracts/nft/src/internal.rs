/// 这个包是合约内部调用的
use crate::*;
use near_sdk::{log, Balance, CryptoHash};
use std::mem::size_of;

pub(crate) fn royalty_to_payout(a: u32, b: Balance) -> U128 {
    U128(a as u128 * b / 10_1000u128)
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl Contract {
    pub(crate) fn assert_ower(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
    }
}
