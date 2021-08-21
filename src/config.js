const CONTRACT_NAME = "worker.testnet";

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
      };
    default:
      throw Error(
        `Unconfigured environment '${env}', Can be configured in src/config.js.`
      );
  }
};

export default getConfig;
