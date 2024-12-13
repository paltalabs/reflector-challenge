import {
  Address,
  Asset,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { toolkitLoader } from "../toolkit";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
} from "soroban-toolkit";

const toolkit = toolkitLoader.getNetworkToolkit("testnet");

async function testTManager() {
  await airdropAccount(toolkit, toolkit.admin);

  let account = await toolkit.horizonRpc.loadAccount(toolkit.admin.publicKey());
  console.log("publicKey", toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Testing Trustless Manager");
  console.log("-------------------------------------------------------");

  const priceResult: any = await invokeContract(
    toolkit,
    "tmanager",
    "get_prices",
    [],
    true
  );
  console.log("🚀 « priceResult:", priceResult);
  console.log("Price Result:", scValToNative(priceResult.result.retval));

  // const configResult: any = await invokeContract(
  //   toolkit,
  //   "tmanager",
  //   "config",
  //   [],
  //   true
  // );
  // console.log("Config Result:", scValToNative(configResult.result.retval));
}

testTManager();
