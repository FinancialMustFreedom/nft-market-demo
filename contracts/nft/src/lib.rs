use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

// 自定义类型
pub type TokenType = String;
pub const CONTRACT_ROYALTY_CAP: u32 = 1000; // 合约版税上限
pub const MINTER_COYALTY_CAP: u32 = 2000; // 铸造版税上限，（nft创造者能收的版税）

near_sdk::setup_alloc!(); // 为wasm32_unknown_unknown优化

/// Helper structure to for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

/*
   near中的nft需要有哪些成员和实现可以去参考
   https://github.com/near/near-sdk-rs/blob/master/near-contract-standards/src/non_fungible_token/core/core_impl.rs
   主要是NEP171，大概有如下几个模块
        - NonFungibleTokenCore -- interface with nft_transfer methods. NonFungibleToken provides methods for it.
        - NonFungibleTokenApproval -- interface with nft_approve methods. NonFungibleToken provides methods for it.
        - NonFungibleTokenEnumeration -- interface for getting lists of tokens. NonFungibleToken provides methods for it.
        - NonFungibleTokenMetadata -- return metadata for the token in NEP-177, up to contract to implement.
   示例可以参考
   https://github.com/near/near-sdk-rs/blob/master/examples/non-fungible-token/nft/src/lib.rs
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] // PanicOnDefault 是一个为Contract生成Default的宏，如果调用default那么会panic
pub struct Contract {
    pub tokens: NonFungibleToken,
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    /// mint nft with id=`token_id` belonging to `token_owner_id
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens
            .mint(token_id, token_owner_id, Some(token_metadata))
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_contract_standards::non_fungible_token::metadata::NFT_METADATA_SPEC;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    const MINT_STORAGE_COST: u128 = 5_870_000_000_000_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    macro_rules! create_default_contract {
        () => {{
            let mut context = get_context(accounts(0));
            testing_env!(context.build());
            let contract = Contract::new(accounts(0).into(), new_default_metadata());
            testing_env!(context
                .storage_usage(env::storage_usage())
                .attached_deposit(MINT_STORAGE_COST)
                .predecessor_account_id(accounts(0))
                .build());
            (context, contract)
        }};
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Near store".into()),
            description: Some("It's an example nft".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    fn new_default_metadata() -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Example Near NFT token".into(),
            symbol: "DOG".into(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_deafult() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        // 我们的Contract有派生类PanicOnDefault，所以调用default的时候会panic
        let _contract = Contract::default();
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1).into(), new_default_metadata());
        testing_env!(context.is_view(true).build()); // 判断是否是只读
        assert_eq!(contract.nft_token("1".to_string()), None); // 判断是否有id为1的nft，这里没有
    }

    #[test]
    fn test_mint() {
        let (_, mut contract) = create_default_contract!();

        let token_id = "0".to_string();
        let token = contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0).to_string());
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let (mut context, mut contract) = create_default_contract!();

        // 铸造
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());

        // 转移
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());

        /* if let 语句，这里是对应于match Option
            match optional {
                Some(i) => {
                    todo!();
                },
                _ => {},
            }
            上面的写法有时候很麻烦，这里用if let,
            下面表示，如果contract.nft_token(token_id.clone())能匹配上Some(token),就执行后面的{},
            如果匹配不上就执行else
        */
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1).to_string());
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    // 关于nft的approve可以参考https://github.com/near/NEPs/discussions/178
    #[test]
    fn test_approve() {
        let (mut context, mut contract) = create_default_contract!();

        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob // alice批准bob转移它的代币
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        // 默认第一个审批id是1， 然后一次递增, 所以这里是Some(1)
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)))
    }

    #[test]
    fn test_revoke() {
        let (mut context, mut contract) = create_default_contract!();
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(1), sample_token_metadata());

        // alice approves bob // alice批准bob转移它的代币
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke(token_id.clone(), accounts(1));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());

        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), None));
    }
}
