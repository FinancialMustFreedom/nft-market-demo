# nft-store

## Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn serve
```

### Compiles and minifies for production
```
yarn build
```

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).

### NFT 合约部署
```bash
// 编译合约
cargo build --all --target wasm32-unknown-unknown --release
// 部署nft
near dev-deploy target/wasm32-unknown-unknown/release/nft.wasm
// 设置合约ID
ID=xxxx
// 初始化nft
near call $ID  new '{"owner_id":"'$ID'","metadata":{"spec":"nft-1.0","name":"nft 2048","symbol":"NFT2048"},"supply_cap_by_type":{"nft-2048":"50000"}, "locked":true}' --accountId=$ID
// 查看owner nft
 near view $ID nft_tokens_for_owner '{"account_id":"wisarmy.testnet", "from_index":"0", "limit":50}'
 ```

### 解锁nft token type
只有解锁的token才可以在市场销售
```bash
$ near call $ID unlock_token_types '{"token_types": ["ntf-2048"]}' --accountId=$ID
Scheduling a call: dev-1630203952408-20676864243759.unlock_token_types({"token_types": ["ntf-2048"]})
Transaction Id 66L69BSRmvuzg8yicfNkx5G457Vrqw1nizeJrWKzJChg
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/66L69BSRmvuzg8yicfNkx5G457Vrqw1nizeJrWKzJChg
''

$ near view $ID get_token_types_locked

```

 ### Market 合约部署
 ```bash
 // 创建maket账号
near create-account market.$ID --masterAccount $ID
// 编译合约
cargo build --all --target wasm32-unknown-unknown --release
// 部署合约
near deploy market.$ID --wasmFile ./target/wasm32-unknown-unknown/release/market.wasm
// 初始化合约
near call market.$ID new '{"owner_id":"market.'$ID'","bid_history_length":3}' --accountId=market.$ID
