import {
  Address,
  Asset,
  xdr,
} from "@stellar/stellar-sdk";
import { toolkitLoader } from "./toolkit";
import {
  airdropAccount,
  deployContract,
  installContract,
} from "soroban-toolkit";

const network = process.argv[2];

const toolkit = toolkitLoader.getNetworkToolkit("testnet");
const soroban_token = new Address(toolkit.addressBook.getContractId("XRP"));

let xlmContractId: string = Asset.native().contractId(toolkit.passphrase);

export async function deployHodlStrategy(
  symbol: string = "XLM",
  contractId: string = ""
) {
  if (network != "mainnet") await airdropAccount(toolkit, toolkit.admin);
  let account = await toolkit.horizonRpc.loadAccount(toolkit.admin.publicKey());
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
    toolkit.admin
  );
}


async function main() {
  try {
    await deployHodlStrategy("XLM", xlmContractId);
    await deployHodlStrategy("XRP", soroban_token.toString());
    toolkit.addressBook.writeToFile();
  } catch (e) {
    console.error(e);
  }
}

main();
