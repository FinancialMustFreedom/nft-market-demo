import Vue from "vue";
import Vuex from "vuex";
import Storage from "@/utils/storage";

Vue.use(Vuex);

const _LOCAL_ = "NEAR_INFO";

export default new Vuex.Store({
  state: {
    nearInfo: Storage.get(_LOCAL_) || {},
  },
  mutations: {
    near(state, status) {
      Storage.set(_LOCAL_, status);
      state.nearInfo = status;
    },
    signOut(state) {
      state.nearInfo.isSignedIn = false;
      Storage.set(_LOCAL_, status);
    },
  },
  actions: {
    updateNear(context, status) {
      context.commit("near", status);
    },
    signOut(context) {
      context.commit("signOut");
    },
  },
  modules: {},
});
