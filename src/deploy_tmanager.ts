import { Address, Asset, xdr } from "@stellar/stellar-sdk";
import { toolkitLoader } from "./toolkit";
import {
  airdropAccount,
  deployContract,
  installContract,
} from "soroban-toolkit";

const toolkit = toolkitLoader.getNetworkToolkit("testnet");

async function deployTManager() {
  await airdropAccount(toolkit, toolkit.admin);

  let account = await toolkit.horizonRpc.loadAccount(toolkit.admin.publicKey());
  console.log("publicKey", toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Trustless Manager");
  console.log("-------------------------------------------------------");

  await installContract(toolkit, "tmanager");

  await deployContract(
    toolkit,
    "tmanager",
    "tmanager",
    [
      new Address(toolkit.addressBook.getContractId("vault")).toScVal(),
      new Address(
        "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63"
      ).toScVal(),
    ],
    toolkit.admin
  );
}

deployTManager();
