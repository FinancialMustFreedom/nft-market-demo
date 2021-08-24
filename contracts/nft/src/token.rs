use crate::*;

pub type TokenId = String;
pub type Payout = HashMap<AccountId, U128>;

/// nft的token本身需要铸造，同时也可以交易，交易需要申请，同时可以有版税收入，并收token类型的供给上限控制
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub owner_id: AccountId, // token 的拥有者
    pub approved_account_ids: HashMap<AccountId, U64>,
    pub next_approval_id: u64, // token交易申请id

    // CUSTOM - fields
    pub royalty: HashMap<AccountId, u32>, // token版税
    pub token_type: Option<String>,       // token类型
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
    pub approved_account_ids: HashMap<AccountId, U64>,

    // 自定义
    pub royalty: HashMap<AccountId, u32>,
    pub token_type: Option<String>,
}
