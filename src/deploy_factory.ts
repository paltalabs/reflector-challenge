import {
  Address,
  Asset,
  nativeToScVal,
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

export async function deployFactory() {
  if (network != "mainnet") await airdropAccount(toolkit, toolkit.admin);
  let account = await toolkit.horizonRpc.loadAccount(toolkit.admin.publicKey());
  console.log("publicKey", toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying defindex factory");
  console.log("-------------------------------------------------------");

  await installContract(toolkit, "factory", undefined, toolkit.admin);
  await installContract(toolkit, "vault", undefined, toolkit.admin);
  const emptyVecScVal = xdr.ScVal.scvVec([]);
  const factoryInitParams: xdr.ScVal[] = [
    new Address(toolkit.admin.publicKey()).toScVal(),
    new Address(toolkit.admin.publicKey()).toScVal(),
    nativeToScVal(50, {type: "u32"}),
    nativeToScVal(Buffer.from(toolkit.addressBook.getWasmHash("vault"), "hex")),
  ];
  await deployContract(
    toolkit,
    "factory",
    `defindex_factory`,
    factoryInitParams,
    toolkit.admin
  );
}


async function main() {
  try {
    await deployFactory();
    toolkit.addressBook.writeToFile();
  } catch (e) {
    console.error(e);
  }
}

main();
