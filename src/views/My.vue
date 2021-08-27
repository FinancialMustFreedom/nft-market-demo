<template>
  <div>
    <h1>This is my nft page</h1>
    <a-row type="flex" justify="space-around" align="middle">
      <a-col :span="5" v-for="(item, i) in myNFTs" :key="i">
        <img :src="item.metadata.media" width="320px" height="220px" />
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

export default {
  name: "mint",
  data() {
    return {
      myNFTs: [],
    };
  },
  created: function () {
    this.getMyNFTs().then((nfts) => {
      console.log("getMyNFTs: ", nfts);
      this.myNFTs = nfts;
    });
  },
  methods: {
    async getMyNFTs() {
      return await window.wallet
        .account()
        .viewFunction(utils.nearConfig.contractName, "nft_tokens_for_owners", {
          account_id: window.wallet.getAccountId(),
          from_index: "0",
          limit: 50,
        });
    },
  },
};
</script>

