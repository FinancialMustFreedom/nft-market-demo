import Vue from "vue";
import VueRouter from "vue-router";
import Home from "../views/Home.vue";
import Store from "../views/Store.vue";
import My from "@/views/My.vue";
import Mint from "@/views/Mint.vue";

Vue.use(VueRouter);

const routes = [
  {
    path: "/",
    name: "Home",
    component: Home,
    children: [
      {
        path: "store",
        name: "store",
        component: Store,
      },
      {
        path: "my",
        component: My,
      },
      {
        path: "mint",
        component: Mint,
      },
    ],
  },
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes,
});

export default router;
