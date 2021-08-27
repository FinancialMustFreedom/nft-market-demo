<template>
  <div>
    <a-form-model
      :rules="rules"
      :model="form"
      :label-col="labelCol"
      :wrapper-col="wrapperCol"
    >
      <a-divider orientation="left">NFT</a-divider>
      <a-form-model-item label="NFT 图片链接" required prop="imageLink">
        <a-input v-model="form.imageLink" @change="changeImageLink()" />
        <img :src="form.imageLink" width="600px" @error="errimg()" />
      </a-form-model-item>
      <a-divider orientation="left">版税</a-divider>
      <a-list
        :grid="{ gutter: 5, column: 3 }"
        size="small"
        bordered
        :data-source="Object.entries(royalties)"
      >
        <a-list-item slot="renderItem" slot-scope="receiverRoyalty">
          <a-card :title="receiverRoyalty[0]" style="margin-top: 5px">
            <a slot="extra" href="#">
              <a-button
                type="danger"
                value="small"
                @click="removeRoyalty(receiverRoyalty[0])"
                style="margin-top: 5px"
              >
                <a-icon type="delete" />
              </a-button>
            </a>
            版税比例: {{ receiverRoyalty[1] }} %
          </a-card>
        </a-list-item>
        <div slot="header">版税列表</div>
        <div slot="footer">
          <a-input-group compact>
            <a-icon slot="prefix" type="user" />
            <a-input
              v-model="form.receiver"
              style="width: 40%"
              default-value=""
              placeholder="Near Account ID"
            />
            <a-input-number
              v-model="form.royalty"
              style="width: 15%"
              :min="0"
              :max="100"
              :step="0.1"
              :parser="(value) => value.replace('%', '')"
              placeholder="版税比例"
              prefix="版税比例"
            />
            <a-button
              type="dashed"
              style="margin-left: 5px"
              @click="addRoyalty()"
            >
              <a-icon type="plus" />添加版税
            </a-button>
          </a-input-group>
        </div>
      </a-list>
      <a-divider></a-divider>
      <a-button @click="handMint">点击铸造</a-button>
    </a-form-model>
  </div>
</template>

<script>
import utils from "../utils/near-utils";

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
      royalties: {},
      imageLinkAvail: false,
      rules: {
        imageLink: [
          { required: true, message: "请输入NFT图片链接", trigger: "blur" },
          { required: true, message: "请输入NFT图片链接", trigger: "change" },
        ],
      },
    };
  },
  methods: {
    error_notification(msg) {
      this.$notification["error"]({
        message: "错误提示",
        description: msg,
      });
    },
    async handMint() {
      if (!this.imageLinkAvail) {
        return this.error_notification("NFT图片链接不是有效的链接");
      }
      let perpetual_royalties = Object.entries(this.royalties)
        .map(([receiver, royalty]) => ({
          [receiver]: royalty * 100,
        }))
        .reduce((acc, cur) => Object.assign(acc, cur), {});
      if (
        Object.values(perpetual_royalties).reduce((a, c) => a + c, 0) > 2000
      ) {
        return this.error_notification("NFT版税总比例不得超过20%");
      }

      let media = this.form.imageLink;
      const metadata = {
        media,
        issued_at: Date.now().toString(),
      };
      const deposit = utils.parseNearAmount("0.1");
      await window.wallet.account().functionCall(
        utils.nearConfig.contractName,
        "nft_mint",
        {
          token_id: utils.nearConfig.tokenType + "-" + Date.now(),
          metadata,
          perpetual_royalties,
          token_type: utils.nearConfig.tokenType,
        },
        utils.nearConfig.GAS,
        deposit
      );
    },
    errimg() {
      if (this.form.imageLink) {
        this.imageLinkAvail = false;
      }
    },
    changeImageLink() {
      this.imageLinkAvail = true;
    },
    async addRoyalty() {
      if (Object.entries(this.royalties).length >= 6) {
        return this.error_notification("最多只能添加6个版税账户");
      }
      if (this.form.receiver) {
        const exists = await utils.isAccountTaken(this.form.receiver);
        if (!exists) {
          return this.error_notification(
            "Near账号 " + this.form.receiver + " 不存在"
          );
        }

        this.$set(this.royalties, this.form.receiver, this.form.royalty);
      }
    },
    removeRoyalty(receiver) {
      this.$delete(this.royalties, receiver);
    },
  },
};
</script>
