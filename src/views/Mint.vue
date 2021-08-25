<template>
  <div>
    <a-form-model
      :rules="rules"
      :model="form"
      :label-col="labelCol"
      :wrapper-col="wrapperCol"
    >
      <a-divider orientation="left">NFT</a-divider>
      <a-form-model-item label="nft图片链接" prop="img">
        <a-input v-model="form.imageLink" />
        <img id="img" :src="form.imageLink" />
        <div>{{ imageLink }}</div>
      </a-form-model-item>
      <a-divider orientation="left">版税</a-divider>
      <a-form-model-item label="账户">
        <a-input v-model="form.accountId" />
      </a-form-model-item>
      <a-form-model-item label="税点">
        <a-input v-model="form.royalty" />
      </a-form-model-item>
      <a-divider></a-divider>
      <a-button @click="handMint">点击铸造</a-button>
    </a-form-model>
  </div>
</template>

<style scoped>
#img {
  width: 600px;
  height: 50%;
}
</style>

<script>
export default {
  name: "mint",
  data() {
    return {
      labelCol: { span: 4 },
      wrapperCol: { span: 14 },
      form: {
        imageLink: "",
        accountId: "",
        royalty: "",
      },
      rules: {
        img: [
          { required: true, message: "请输入nft图片链接", trigger: "change" },
        ],
      },
    };
  },
  methods: {
    handMint: function () {},
  },
};
</script>
