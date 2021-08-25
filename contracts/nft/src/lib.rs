use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

use near_sdk::{env, near_bindgen, AccountId, CryptoHash, PanicOnDefault, StorageUsage};

pub use crate::enumerable::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::nft_core::*;
pub use crate::token::*;

mod enumerable;
mod internal;
mod metadata;
mod nft_core;
mod token;

// 自定义类型
pub type TokenType = String;
pub type TypeSupplyCaps = HashMap<TokenType, U64>;
pub const CONTRACT_ROYALTY_CAP: u32 = 1000;
pub const MINTER_ROYALTY_CAP: u32 = 2000;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,                                           // 合约地址
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, // 通过account_id 查询他所拥有的token
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, // 通过tokenid查询token的metadata基本信息
    pub tokens_by_id: LookupMap<TokenId, Token>,                    // 通过tokenid查询token
    pub extra_storage_in_bytes_per_token: StorageUsage,             // 每个新token字节单位的存储大小
    pub metadata: LazyOption<NFTMetadata>,

    // 自定义部分
    pub supply_cap_by_type: TypeSupplyCaps, // 每种token的铸币上限
    pub tokens_per_type: LookupMap<TokenType, UnorderedSet<TokenId>>, // 记录每种token的数量
    pub token_types_locked: UnorderedSet<TokenType>,
    pub contract_royalty: u32, // 合约所收的版税
}

// storage key
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NftMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        metadata: NFTMetadata,
        supply_cap_by_type: TypeSupplyCaps,
        locked: Option<bool>,
    ) -> Self {
        let mut this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            owner_id: owner_id.into(),
            extra_storage_in_bytes_per_token: 0,
            metadata: LazyOption::new(
                StorageKey::NftMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            supply_cap_by_type,
            tokens_per_type: LookupMap::new(StorageKey::TokensPerType.try_to_vec().unwrap()),
            token_types_locked: UnorderedSet::new(
                StorageKey::TokenTypesLocked.try_to_vec().unwrap(),
            ),
            contract_royalty: 0,
        };
        if locked.unwrap_or(false) {
            // 默认是locked
            for token_type in this.supply_cap_by_type.keys() {
                this.token_types_locked.insert(&token_type);
            }
        }
        this.measure_min_token_storage_cost();
        this
    }

    /// 计算存储token的最小消耗, 创建一个新的账户计算存储消耗后删除
    fn measure_min_token_storage_cost(&mut self) {
        let initial_storage_usage = env::storage_usage(); // 当前的存储使用量
        let tmp_account_id = "a".repeat(64); // 创建一个64个"a"的账户id
        let u = UnorderedSet::new(
            StorageKey::TokenPerOwnerInner {
                account_id_hash: hash_account_id(&tmp_account_id),
            }
            .try_to_vec()
            .unwrap(),
        );
        self.tokens_per_owner.insert(&tmp_account_id, &u); // 插入tokens_per_owern

        let tokens_per_owner_entry_in_bytes = env::storage_usage() - initial_storage_usage; // 计算差值获取一个账户的消耗
        let owner_id_extra_cost_in_bytes = (tmp_account_id.len() - self.owner_id.len()) as u64;

        self.extra_storage_in_bytes_per_token =
            tokens_per_owner_entry_in_bytes + owner_id_extra_cost_in_bytes;

        self.tokens_per_owner.remove(&tmp_account_id);
    }

    /// nft的铸造这里先介绍一些简单的知识
    /// 1. current_account_id
    ///     当前合约的拥有者账户id
    /// 2. singer_account_id
    ///     签署原始交易或初始化夸合约调用的帐户的ID。
    /// 3. predecessor_account_id
    ///     作为跨合同调用链中的前一个合同的帐户ID。如果他是第一个合约，那么他等于signer_account_id
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: Option<TokenId>,
        metadata: TokenMetadata, // token 的基本信息
        perpetual_royalties: Option<HashMap<AccountId, u32>>, // 一个作品可以有多个作者，所以可能有多人收税
        receiver_id: Option<ValidAccountId>,
        token_type: Option<TokenType>,
    ) {
        let mut final_token_id = format!("{}", self.token_metadata_by_id.len() + 1); // tokenid是从1不断递增
        if let Some(token_id) = token_id {
            // 如果传人token_id就使用传入
            final_token_id = token_id
        }

        let initial_storage_usage = env::storage_usage(); // 记录当前的存储使用情况
        let mut owner_id = env::predecessor_account_id();
        if let Some(receiver_id) = receiver_id {
            owner_id = receiver_id.into();
        }

        // 自定义创建版税map
        let mut royalty = HashMap::new();
        let mut total_perpetual = 0;
        if let Some(perpetual_royalties) = perpetual_royalties {
            assert!(
                // 如果传入了版税信息，那么判断收税人是否多于5个，超过了不支持
                perpetual_royalties.len() < 6,
                "Cannot add more then 6 perpetual royalty amounts"
            );
            // 将传入的版税信息写如到合约
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
                total_perpetual += amount; // 累计这个作品的总版税
            }
        }
        // 总版税现在在20%以内
        assert!(
            total_perpetual <= MINTER_ROYALTY_CAP,
            "Perpetual royalties cannot be more then 20%"
        );

        // 自定义 按TOKEN_TYPE强制实施铸币上限
        if token_type.is_some() {
            let token_type = token_type.clone().unwrap();
            let cap = u64::from(
                *self
                    .supply_cap_by_type // 支持的token类型和上限在合约铸造的时候给定了，当然也可以后面调用add_token_type函数添加
                    .get(&token_type)
                    .expect("Token type must have supply cap"),
            );
            let supply = u64::from(self.nft_supply_for_type(&token_type));
            assert!(supply < cap, "Cannot mint anymore token type."); // 供给量不能大于上限
            let mut tokens_per_type = self.tokens_per_type.get(&token_type).unwrap_or_else(|| {
                UnorderedSet::new(
                    // 没有就新建
                    StorageKey::TokensPerTypeInner {
                        token_type_hash: hash_account_id(&token_type),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
            tokens_per_type.insert(&final_token_id);
            self.tokens_per_type.insert(&token_type, &tokens_per_type);
        }

        let token = Token {
            owner_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
            royalty,
            token_type,
        };
        assert!(
            self.tokens_by_id.insert(&final_token_id, &token).is_none(), // 添加tokenid对token的索引
            "Token already exists"
        );
        self.token_metadata_by_id.insert(&final_token_id, &metadata); // 添加索引
        self.internal_add_token_to_owner(&token.owner_id, &final_token_id);

        let new_token_size_in_bytes = env::storage_usage() - initial_storage_usage;
        let required_storage_in_bytes =
            self.extra_storage_in_bytes_per_token + new_token_size_in_bytes;

        refund_deposit(required_storage_in_bytes); // 返回多余的钱
    }

    // 一些自定义的setters
    // self是一个语法糖， self表示  self: Self; &self表示&Self； &mut self表示&mut Self
    pub fn set_contract_royalty(&mut self, contract_royalty: u32) {
        self.assert_ower();
        assert!(
            contract_royalty <= CONTRACT_ROYALTY_CAP,
            "Contract royalties limited to 10% for owner"
        );
        self.contract_royalty = contract_royalty;
    }

    /// 添加token类型
    pub fn add_token_types(&mut self, supply_cap_by_type: TypeSupplyCaps, locked: Option<bool>) {
        self.assert_ower();
        for (token_type, hard_cap) in &supply_cap_by_type {
            if locked.unwrap_or(false) {
                assert!(
                    self.token_types_locked.insert(&token_type),
                    "Token type should not be locked"
                );
            }
            assert!(
                self.supply_cap_by_type
                    .insert(token_type.to_string(), *hard_cap)
                    .is_none(),
                "Token type exists"
            );
        }
    }

    pub fn unlock_token_types(&mut self, token_types: Vec<String>) {
        for token_type in &token_types {
            self.token_types_locked.remove(&token_type);
        }
    }

    // 自定义的views方法
    pub fn get_contract_royalty(&self) -> u32 {
        self.contract_royalty
    }
    pub fn get_supply_caps(&self) -> TypeSupplyCaps {
        self.supply_cap_by_type.clone()
    }
    pub fn get_token_types_locked(&self) -> Vec<String> {
        self.token_types_locked.to_vec()
    }
    pub fn is_token_locked(&self, token_id: TokenId) -> bool {
        let token = self.tokens_by_id.get(&token_id).expect("No token");
        assert!(token.token_type.is_some(), "Token must have type");
        let token_type = token.token_type.unwrap();
        self.token_types_locked.contains(&token_type)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use core::panic;

    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    const MINT_STORAGE_COST: u128 = 50_000_000_000_000_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_default_nft_metadata() -> NFTMetadata {
        NFTMetadata {
            spec: "nft-1.0".to_string(),
            name: "market nft".to_string(),
            symbol: "MNFT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }
    fn get_default_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("test token".to_string()),
            description: Some("test token desc".to_string()),
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    fn get_default_contract(supply_cap_by_type: TypeSupplyCaps) -> Contract {
        Contract::new(
            accounts(1).into(),
            get_default_nft_metadata(),
            supply_cap_by_type,
            Some(false),
        )
    }

    #[test]
    fn hm() {
        let mut hm = HashMap::new();
        hm.insert(1, 2);
        assert_eq!(hm.get(&1).unwrap().clone(), 2);
        hm.insert(1, 3);
        assert_eq!(hm.get(&1).unwrap().clone(), 3);
    }

    #[test]
    #[should_panic]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = get_default_contract(HashMap::new());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    /// 测试铸造
    #[test]
    fn test_nft_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());

        let token_type = "t_token".to_string();
        let mut supply_cap = HashMap::new();
        supply_cap.insert(token_type.clone(), U64::from(10u64));
        let mut contract = get_default_contract(supply_cap);

        let token_id = "0".to_string();
        let mut perpetual_royalties = HashMap::new();
        let r = 20u32;
        perpetual_royalties.insert(accounts(0).into(), r);

        testing_env!(context.attached_deposit(MINT_STORAGE_COST).build());
        contract.nft_mint(
            Some(token_id.clone()),
            get_default_token_metadata(),
            Some(perpetual_royalties),
            Some(accounts(0).into()),
            Some(token_type.clone()),
        );

        let token_json = contract.nft_token(token_id.clone()).unwrap();
        assert_eq!(token_json.token_id, token_id);
        assert_eq!(token_json.token_type.unwrap(), token_type);
        let rr = token_json.royalty.get(accounts(0).as_ref()).unwrap();
        assert_eq!(*rr, r);
    }

    /// 测试转币
    #[test]
    fn test_nft_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());

        let token_type = "t_token".to_string();
        let mut supply_cap = HashMap::new();
        supply_cap.insert(token_type.clone(), U64::from(10u64));
        let mut contract = get_default_contract(supply_cap);

        let token_id = "0".to_string();
        let mut perpetual_royalties = HashMap::new();
        let r = 20u32;
        perpetual_royalties.insert(accounts(0).into(), r);

        testing_env!(context.attached_deposit(MINT_STORAGE_COST).build());
        // 给accounts(0)铸造了一个token_id=0的nft
        contract.nft_mint(
            Some(token_id.clone()),
            get_default_token_metadata(),
            Some(perpetual_royalties),
            Some(accounts(0).into()),
            Some(token_type.clone()),
        );

        testing_env!(context.attached_deposit(1).build()); // 转币要质押
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context.is_view(true).attached_deposit(0).build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_ne!(token.owner_id, accounts(0).to_string());
            assert_eq!(token.owner_id, accounts(1).to_string());
            assert_eq!(token.approved_account_ids, HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }
}
