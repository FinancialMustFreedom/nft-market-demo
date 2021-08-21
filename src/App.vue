<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script>
import utils from "@/utils/near-utils";
const { getWallet } = utils;

const initNear = async () => {
  const { near, wallet, accountContract } = await getWallet();
  const signedIn = wallet.isSignedIn();
  window.wallet = wallet;
  const info = {
    near: near,
    accountContract: accountContract,
    accountId: wallet.getAccountId(),
    isSignedIn: signedIn,
  };
  return info;
};

export default {
  name: "app",
  created: function () {
    initNear().then((info) => {
      this.$store.dispatch("updateNear", info);
    });
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
