import { Address, Asset, nativeToScVal, Networks, xdr } from "@stellar/stellar-sdk";
import { toolkitLoader } from "./toolkit";
import { airdropAccount, deployContract, installContract, invokeContract,  } from "soroban-toolkit";

const toolkit = toolkitLoader.getNetworkToolkit("testnet");


let xlmContractId: string = Asset.native().contractId(toolkit.passphrase);

export async function deployHdolStrategy(symbol: string = "XLM", contractId: string = "") {
  if (network != "mainnet") await airdropAccount(toolkit, toolkit.admin);
  let account = await toolkit.horizonRpc.loadAccount(
    toolkit.admin.publicKey()
  );
  console.log("publicKey", toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Hodl Strategy");
  console.log("-------------------------------------------------------");

  await installContract(toolkit, "hodl_strategy", undefined, toolkit.admin);
  const emptyVecScVal = xdr.ScVal.scvVec([]);
  const addressScVal = new Address(contractId).toScVal();
  await deployContract(
    toolkit,
    "hodl_strategy",
    `hodl_${symbol.toLowerCase()}`,
    [addressScVal, emptyVecScVal],
    toolkit.admin,
  );
}

const network = process.argv[2];

const soroban_token = new Address(toolkit.addressBook.getContractId("soroban_token"))
async function main(){
  try {
    await deployHdolStrategy("XLM", xlmContractId);
    await deployHdolStrategy("XRP", soroban_token.toString());
    toolkit.addressBook.writeToFile();
  } catch (e) {
    console.error(e);
  }
}

main();
