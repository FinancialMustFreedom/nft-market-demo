<template>
  <div>
    <a-row type="flex" justify="space-around" align="middle">
      <a-col :span="5" v-for="(item, i) in metadatas" :key="i">
        <img :src="item.md[0].metadata.media" width="320px" height="220px" />
        <div id="on-sale">售价：{{ item.show_price }} near</div>
        <a-button
          id="buy"
          @click="buy_nft(item.md[0].token_id, item.show_price)"
          >购买</a-button
        >
        <a-divider></a-divider>
      </a-col>
    </a-row>
  </div>
</template>

<script>
import utils from "../utils/near-utils";
import {
  formatNearAmount,
  parseNearAmount,
} from "near-api-js/lib/utils/format";

const getSales = async () => {
  const sales = await window.wallet
    .account()
    .viewFunction(
      utils.marketNearConfig.contractName,
      "get_sales_by_nft_contract_id",
      {
        nft_contract_id: utils.nearConfig.contractName,
        from_index: "0",
        limit: 50,
      }
    );
  return sales;
};

const getMetadata = async (token_id) => {
  const md = await window.wallet
    .account()
    .viewFunction(utils.nearConfig.contractName, "nft_tokens_batch", {
      token_ids: [token_id],
    });
  return md;
};

export default {
  name: "store",
  created: function () {
    getSales().then((sales) => {
      this.sales = sales;
      for (let i in sales) {
        getMetadata(sales[i].token_id).then((md) => {
          this.metadatas.push({
            md,
            price: sales[i].sale_conditions.near,
            show_price: formatNearAmount(sales[i].sale_conditions.near, 2),
          });
        });
      }
    });
  },
  data() {
    return {
      // sales: [],
      metadatas: [],
    };
  },
  methods: {
    buy_nft: async function (token_id, offerPrice) {
      console.log("buy, token_id: ", token_id, " offerPrice", offerPrice);
      if (offerPrice == 0) {
        offerPrice = "0.0001";
      }
      await window.wallet
        .account()
        .functionCall(
          utils.marketNearConfig.contractName,
          "offer",
          {
            nft_contract_id: utils.nearConfig.contractName,
            token_id: token_id,
          },
          utils.nearConfig.GAS,
          parseNearAmount(offerPrice)
        )
        .then(() => {
          this.$set();
        });
    },
  },
};
</script>

<style scoped>
#h220 {
  height: 220px;
  background: greenyellow;
}
#on-sale {
  margin-top: 10px;
  margin-left: 40px;
  float: left;
}
#buy {
  margin-top: 10px;
}
</style>
