use std::mem::size_of;

/// 这个包是合约内部调用的
use crate::*;
use near_sdk::{log, Balance, CryptoHash, Promise};

pub(crate) fn royalty_to_payout(a: u32, b: Balance) -> U128 {
    U128(a as u128 * b / 10_1000u128)
}

/// 断言是否附带 1 yoctoNEAR
pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR"
    )
}

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

/// 返还多给的钱
pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost
    );
    let refund = attached_deposit - required_cost;
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    account_id.len() as u64 + 4 + size_of::<u64>() as u64
}

pub(crate) fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_account_ids: I,
) -> Promise
where
    I: Iterator<Item = &'a AccountId>,
{
    let storage_released: u64 = approved_account_ids
        .map(bytes_for_approved_account_id)
        .sum();
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

pub(crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, U64>,
) -> Promise {
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

impl Contract {
    /// 判断函数调用者是否是合约拥有者
    pub(crate) fn assert_ower(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
    }

    /// 添加token记录
    /// 如果这个人拥有其他的token就直接往里面加，如果是第一个就新建一个map
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        tokens_set.insert(token_id);
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        // 获取账户的所有token
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            .expect("Token should be owned by the sender");
        tokens_set.remove(token_id);
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    /// 转移token
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        approval_id: Option<U64>,
        memo: Option<String>,
    ) -> Token {
        let token = self.tokens_by_id.get(token_id).expect("Token not found");

        // 自定义部分, token_type can be locked untill unlocked by owner
        if token.token_type.is_some() {
            assert_eq!(
                self.token_types_locked
                    .contains(&token.token_type.clone().unwrap()),
                false,
                "Token transfer are locked"
            );
        }

        if sender_id != &token.owner_id && !token.approved_account_ids.contains_key(sender_id) {
            env::panic(b"Unauthorized");
        }

        // 如果包含 enforce_approval_id，请检查 receiver approval id
        if let Some(enforced_approval_id) = approval_id {
            let actual_approval_id = token
                .approved_account_ids
                .get(sender_id)
                .expect("Sender is not approved account");
            assert_eq!(
                actual_approval_id, &enforced_approval_id,
                "The actual approval_id {} is different from the given approval_id {}",
                actual_approval_id.0, enforced_approval_id.0
            );
        }

        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        log!(
            "Transfer {} from @{} to @{}",
            token_id,
            &token.owner_id,
            receiver_id
        );

        // 从原拥有者删除
        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        // 加入到新拥有者里
        self.internal_add_token_to_owner(receiver_id, token_id);

        let new_token = Token {
            owner_id: receiver_id.clone(),
            approved_account_ids: Default::default(),
            next_approval_id: token.next_approval_id,
            royalty: token.royalty.clone(),
            token_type: token.token_type.clone(),
        };
        self.tokens_by_id.insert(token_id, &new_token);

        if let Some(memo) = memo {
            env::log(format!("Memo: {}", memo).as_bytes());
        }

        token
    }
}
