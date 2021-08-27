<template>
  <div>
    <h1>This is my nft page</h1>
    <a-alert
      v-if="storagePaid == 0"
      message="还未在市场注册销售"
      type="warning"
    >
      <p slot="description">
        <a-button
          type="primary"
          :loading="storePaidLoading"
          @click="marketRegisterStorage()"
        >
          注册市场销售
        </a-button>
      </p>
    </a-alert>
    <a-row type="flex" justify="space-around" align="middle">
      <a-col :span="10" v-for="(item, i) in myNFTs" :key="i">
        <div style="display: none">{{ (item.sales_ft = "near") }}</div>
        <img :src="item.metadata.media" width="200px" height="220px" />
        <div v-show="storagePaid > 0">
          <a-input-group compact>
            <a-select default-value="near" v-model="item.sales_ft">
              <a-select-option value="near"> NEAR </a-select-option>
            </a-select>
            <a-input-number
              v-model="item.sales_price"
              style="width: 30%"
              :min="0"
              :step="0.1"
              :parser="(value) => value.replace('%', '')"
              placeholder="价格"
              prefix="价格"
            />
            <a-button
              type="primary"
              :loading="saleLoading"
              @click="marketSaleSubmit(i)"
            >
              出售
            </a-button>
          </a-input-group>
        </div>
        <a-divider></a-divider>
      </a-col>
    </a-row>
  </div>
</template>

<style scoped>
#h220 {
  height: 220px;
  background: orange;
}
</style>
<script>
import utils from "../utils/near-utils";
import BN from "bn.js";

export default {
  name: "mint",
  data() {
    return {
      myNFTs: [],
      storagePaid: -1,
      storePaidLoading: false,
      saleLoading: false,
    };
  },
  beforeCreate() {},
  created() {
    this.getMyNFTs().then((nfts) => {
      console.log("getMyNFTs: ", nfts);
      this.myNFTs = nfts;
    });
    this.marketStoragePaid().then((storagePaid) => {
      this.storagePaid = storagePaid;
    });
  },
  computed: {},
  methods: {
    async getMyNFTs() {
      return await window.wallet
        .account()
        .viewFunction(utils.nearConfig.contractName, "nft_tokens_for_owner", {
          account_id: window.wallet.getAccountId(),
          from_index: "0",
          limit: 50,
        });
    },
    async marketStoragePaid() {
      return await window.wallet
        .account()
        .viewFunction(utils.marketNearConfig.contractName, "storage_paid", {
          account_id: window.wallet.getAccountId(),
        });
    },
    async marketRegisterStorage() {
      this.storePaidLoading = true;
      return await window.wallet
        .account()
        .functionCall(
          utils.marketNearConfig.contractName,
          "storage_deposit",
          {},
          utils.nearConfig.GAS,
          new BN(
            await window.wallet
              .account()
              .viewFunction(
                utils.marketNearConfig.contractName,
                "storage_amount",
                {},
                utils.nearConfig.GAS
              )
          ).mul(new BN("10"))
        );
    },
    marketSaleSubmit(index) {
      this.handleSaleUpdate(
        this.myNFTs[index].token_id,
        this.myNFTs[index].sales_ft,
        this.myNFTs[index].sales_price + ""
      );
    },
    async handleSaleUpdate(token_id, ft, price) {
      this.saleLoading = true;
      console.log(token_id, ft, price);
      const sale = await window.wallet
        .account()
        .viewFunction(utils.marketNearConfig.contractName, "get_sale", {
          nft_contract_token: utils.nearConfig.contractName + ":" + token_id,
        })
        .catch(() => {});

      if (sale) {
        await window.wallet.account().functionCall(
          utils.marketNearConfig.contractName,
          "update_price",
          {
            nft_contract_id: utils.nearConfig.contractName,
            token_id,
            ft_token_id: ft,
            price: utils.parseNearAmount(price),
          },
          utils.nearConfig.GAS
        );
      } else {
        let sale_conditions = {
          near: utils.parseNearAmount(price),
        };
        await window.wallet.account().functionCall(
          utils.nearConfig.contractName,
          "nft_approve",
          {
            token_id,
            account_id: utils.marketNearConfig.contractName,
            msg: JSON.stringify({ sale_conditions }),
          },
          utils.nearConfig.GAS,
          utils.parseNearAmount("0.01")
        );
      }
    },
  },
};
</script>

