<template>
  <div>
    <h1>This is an store page</h1>
    <a-row type="flex" justify="space-around" align="middle">
      <a-col :span="5" v-for="(item, i) in metadatas" :key="i">
        <img :src="item[0].metadata.media" width="320px" height="220px" />
        <a-divider></a-divider>
      </a-col>
    </a-row>
  </div>
</template>

<script>
import utils from "../utils/near-utils";

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
  console.log("->>>>>>> token: ", token_id, " md: ", md);
  return md;
};

export default {
  name: "store",
  created: function () {
    getSales().then((sales) => {
      this.sales = sales;
      console.log("---> sales: ", sales);
      let i;
      for (i in sales) {
        getMetadata(sales[i].token_id).then((md) => {
          this.metadatas.push(md);
          console.log("-----> md: ", md);
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
};
</script>

<style scoped>
#h220 {
  height: 220px;
  background: greenyellow;
}
</style>
