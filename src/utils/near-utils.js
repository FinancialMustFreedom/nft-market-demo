import { Account, keyStores, Near, WalletConnection } from "near-api-js";
import getConfig from "../config";
import { formatNearAmount } from "near-api-js/lib/utils/format";

const nearConfig = getConfig("testnet");

const near = new Near({
  nodeUrl: nearConfig.nodeUrl,
  walletUrl: nearConfig.walletUrl,
  networkId: nearConfig.networkId,
  deps: {
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  },
});
const getWallet = async () => {
  const wallet = new WalletConnection(near);
  const contractAccount = new Account(near.connection, nearConfig.contractName);
  return { near, wallet, contractAccount };
};

const getBalance = async ({ wallet }) => {
  return formatNearAmount(
    (await wallet.account().getAccountBalance()).available,
    4
  );
};
export default { getWallet, getBalance };
