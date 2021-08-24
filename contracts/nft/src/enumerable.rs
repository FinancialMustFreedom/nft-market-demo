use crate::*;

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
}
