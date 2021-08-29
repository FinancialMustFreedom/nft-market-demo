const CONTRACT_NAME = "dev-1630203952408-20676864243759";

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
