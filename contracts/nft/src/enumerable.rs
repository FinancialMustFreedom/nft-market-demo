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
}
