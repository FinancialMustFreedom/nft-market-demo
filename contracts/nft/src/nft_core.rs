use crate::*;
use near_sdk::{env, ext_contract, Balance, Gas};

const NO_DEPOSIT: Balance = 0;
const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;

pub trait NonFungibleTokenCore {
    /// 通过tokenid获取Jons格式的token
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;

    /// 转移token
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<U64>,
        memo: Option<String>, // MEMO是memorandum的缩写,指“便函,便笺”
    );

    /// 申请nft转移
    fn nft_approve(&mut self, token_id: TokenId, account_id: ValidAccountId, msg: Option<String>);

    /// 取消nft转移
    fn nft_revoke(&mut self, token_id: TokenId, account_id: ValidAccountId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: U64,
        msg: String,
    ) -> Promise;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<U64>,
        memo: Option<String>,
    ) {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let previous_token = self.internal_transfer(
            &sender_id,
            receiver_id.as_ref(),
            &token_id,
            approval_id,
            memo,
        );
        refund_approved_account_ids(
            previous_token.owner_id.clone(),
            &previous_token.approved_account_ids,
        );
    }

    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            Some(JsonToken {
                token_id,
                owner_id: token.owner_id,
                metadata,
                royalty: token.royalty,
                approved_account_ids: token.approved_account_ids,
                token_type: token.token_type,
            })
        } else {
            None
        }
    }

    /// nft_approve表示self批准account_id转移他的代币
    /// 关于approve可以参考https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: ValidAccountId, msg: Option<String>) {
        assert_at_least_one_yocto();
        /* 关于from和into，他们是一对用于类型转换的函数，就像下面这种into
            into:
                let account_id: AccountId = account_id.into();
                前面声明了account_id是AccountId类型，而account_id是ValidAccountId类型，这里调用into,就是把ValidAccountId转换为AccountId
           form:
                let s = String::from("apply");
                上面声明了s但是不像into里的account_id直接指明类型，而是在等号后面String这里表明类型，
                把"apply"转换为String
            总结：from和into都是用来转换类型的，不同的地方是要转换的类型一个在=前面声明一个在后面
        */
        let account_id: AccountId = account_id.into();
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");
        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Predecessor must be the token owner."
        );

        let approval_id: U64 = token.next_approval_id.into();
        let is_new_approval = token
            .approved_account_ids
            .insert(account_id.clone(), approval_id)
            .is_none();

        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        token.next_approval_id += 1;
        self.tokens_by_id.insert(&token_id, &token); // 这里相当于更新了token的next_approval_id信息，同一个key插入两次是没问题的

        refund_deposit(storage_used);

        if let Some(msg) = msg {
            let mut final_msg = msg;
            let token_type = token.token_type;
            if let Some(token_type) = token_type {
                final_msg.insert_str(
                    final_msg.len() - 1,
                    &format!(", \"token_type\":\"{}\"", token_type),
                );

                ext_non_fungible_approval_receiver::nft_on_approve(
                    token_id,
                    token.owner_id,
                    approval_id,
                    final_msg,
                    &account_id,
                    NO_DEPOSIT,
                    env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
                )
                .as_return();
            }
        }
    }

    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: ValidAccountId) {
        assert_one_yocto();
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);
        if token
            .approved_account_ids
            .remove(account_id.as_ref())
            .is_some()
        {
            refund_approved_account_ids_iter(predecessor_account_id, [account_id.into()].iter());
            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}
