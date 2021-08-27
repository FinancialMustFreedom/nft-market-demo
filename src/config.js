const CONTRACT_NAME = "dev-1624406486386-79437689012031";
// const CONTRACT_NAME = "dev-1630028625884-87147957230520";

const getConfig = (env) => {
  switch (env) {
    case "testnet":
      return {
        networkId: "testnet",
        nodeUrl: "https://rpc.testnet.near.org",
        contractName: CONTRACT_NAME,
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
        GAS: "200000000000000",
        tokenType: "nft-2048",
      };
    default:
      throw Error(
        `Unconfigured environment '${env}', Can be configured in src/config.js.`
      );
  }
};

export default getConfig;
