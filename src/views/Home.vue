<template>
  <a-layout id="components-layout-demo-top" class="layout">
    <a-layout-header>
      <div id="sign">
        <a-button type="primary" v-if="updateSign" @click="signOut">{{
          updateAccount
        }}</a-button>
        <a-button type="danger" v-else @click="signIn">登陆</a-button>
      </div>
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

<style>
#sign {
  height: 31px;
  margin: 2px 5px 16px 0;
  float: left;
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
