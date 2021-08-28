<template>
  <div>
    <a-list
      item-layout="vertical"
      size="small"
      :pagination="pagination"
      :data-source="myNFTs"
    >
      <div slot="header">
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
      </div>
      <a-list-item slot="renderItem" key="item.title" slot-scope="item, index">
        <a-row type="flex" justify="space-around" align="middle">
          <a-col>
            <div style="display: none">
              {{ (item.sales_ft = "near") }}
              {{ (item.sales_price = 0.0) }}
            </div>
            <img :src="item.metadata.media" width="300" />
          </a-col>
          <a-col>
            <a-list-item-meta description="版税"> </a-list-item-meta>
            <p v-for="(royalty, receiver) in item.royalty" :key="receiver">
              <span>{{ receiver }}</span>
              <a-divider type="vertical" dashed />
              <span>{{ royalty / 100 }} %</span>
            </p>
            <div v-show="storagePaid > 0">
              <a-input-group compact>
                <a-select default-value="near" v-model="item.sales_ft">
                  <a-select-option value="near"> NEAR </a-select-option>
                </a-select>
                <a-input-number
                  v-model="item.sales_price"
                  style="width: 30%"
                  :default-value="0.0"
                  :min="0"
                  :step="0.1"
                  placeholder="价格"
                  prefix="价格"
                />
                <a-button
                  type="primary"
                  :loading="saleLoading[index]"
                  @click="marketSaleSubmit(index)"
                >
                  出售
                </a-button>
              </a-input-group>
            </div>
          </a-col>
        </a-row>
      </a-list-item>
    </a-list>
  </div>
</template>

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
      saleLoading: [],
    };
  },
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
      this.$set(this.saleLoading, index, true);
      this.handleSaleUpdate(
        this.myNFTs[index].token_id,
        this.myNFTs[index].sales_ft,
        this.myNFTs[index].sales_price + ""
      );
    },
    async handleSaleUpdate(token_id, ft, price) {
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

