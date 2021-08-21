<template>
  <a-layout id="components-layout-demo-top" class="layout">
    <a-layout-header>
      <div id="sign">
        <a-button type="primary" v-if="updateSign" @click="signOut">{{
          updateAccount
        }}</a-button>
        <a-button type="danger" v-else @click="signIn">登陆</a-button>
      </div>
      <div id="menu">
        <a-menu
          theme="dark"
          mode="horizontal"
          :default-selected-keys="['1']"
          :style="{ lineHeight: '64px' }"
        >
          <a-menu-item key="1">
            <router-link to="store"> NFT 商场 </router-link>
          </a-menu-item>
          <a-menu-item key="2">
            <router-link to="my">我的NFT</router-link>
          </a-menu-item>
        </a-menu>
      </div>
      <div id="upload">
        <router-link to="mint"
          ><a-button type="danger">铸造</a-button></router-link
        >
      </div>
    </a-layout-header>

    <a-layout-content style="padding: 0 50px">
      <div :style="{ background: '#fff', padding: '24px', minHeight: '480px' }">
        <!-- Content -->
        <router-view></router-view>
      </div>
    </a-layout-content>

    <a-layout-footer style="text-align: center">
      NFT商场 ©2021 Created by ZCSL
    </a-layout-footer>
  </a-layout>
</template>

<style scoped>
#sign {
  height: 31px;
  margin: 0px 5px 15px 0;
  float: left;
}
#menu {
  margin-left: 40%;
  float: left;
}
#upload {
  height: 31px;
  margin: 0px 5px 15px 0;
  float: right;
}
</style>

<script>
// @ is an alias to /src
import getConfig from "@/config";

const nearConfig = getConfig("testnet");

export default {
  name: "Home",
  components: {},
  methods: {
    signIn: function () {
      if (!this.$store.state.isSignedIn) {
        window.wallet.requestSignIn(nearConfig.accountId, "nft store");
      }
    },
    signOut: function () {
      window.wallet.signOut();
      this.$store.dispatch("signOut");
    },
  },
  computed: {
    updateSign() {
      return this.$store.state.nearInfo.isSignedIn;
    },
    updateAccount() {
      return this.$store.state.nearInfo.accountId;
    },
  },
};
</script>
