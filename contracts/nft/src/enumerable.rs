use crate::*;
use core::cmp::min;

#[near_bindgen]
impl Contract {
    pub fn nft_supply_for_type(&self, token_type: &String) -> U64 {
        let tokens_per_type = self.tokens_per_type.get(&token_type);
        if let Some(tokens_per_type) = tokens_per_type {
            U64(tokens_per_type.len())
        } else {
            U64(0)
        }
    }

    pub fn nft_tokens(&self, from_index: U64, limit: u64) -> Vec<JsonToken> {
        let mut tmp = vec![];
        let keys = self.token_metadata_by_id.keys_as_vector();
        let start = u64::from(from_index);
        let end = min(start + limit, keys.len());
        for i in start..end {
            tmp.push(self.nft_token(keys.get(i).unwrap()).unwrap());
        }
        tmp
    }

    pub fn nft_tokens_batch(&self, token_ids: Vec<String>) -> Vec<JsonToken> {
        let mut tmp = vec![];
        for i in 0..token_ids.len() {
            tmp.push(self.nft_token(token_ids[i].clone()).unwrap());
        }
        tmp
    }
    pub fn nft_tokens_for_type(
        &self,
        token_type: String,
        from_index: U64,
        limit: u64,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        let tokens_per_type = self.tokens_per_type.get(&token_type);
        let tokens = if let Some(tokens_per_type) = tokens_per_type {
            tokens_per_type
        } else {
            return vec![];
        };
        let keys = tokens.as_vector();
        let start = u64::from(from_index);
        let end = min(start + limit, keys.len());
        for i in start..end {
            tmp.push(self.nft_token(keys.get(i).unwrap()).unwrap());
        }
        tmp
    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U64 {
        let tokens_owner = self.tokens_per_owner.get(&account_id);
        if let Some(tokens_owner) = tokens_owner {
            U64(tokens_owner.len())
        } else {
            U64(0)
        }
    }
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: U64,
        limit: u64,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        let tokens_owner = self.tokens_per_owner.get(&account_id);
        let tokens = if let Some(tokens_owner) = tokens_owner {
            tokens_owner
        } else {
            return vec![];
        };
        let keys = tokens.as_vector();
        let start = u64::from(from_index);
        let end = min(start + limit, keys.len());
        for i in start..end {
            tmp.push(self.nft_token(keys.get(i).unwrap()).unwrap());
        }
        tmp
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
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

    fn get_default_contract(supply_cap_by_type: TypeSupplyCaps) -> Contract {
        Contract::new(
            accounts(1).into(),
            get_default_nft_metadata(),
            supply_cap_by_type,
            Some(false),
        )
    }

    #[test]
    fn test_token() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = get_default_contract(HashMap::new());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }
}
