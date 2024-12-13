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

  const assetRatios = [
    {
      asset: toolkit.addressBook.getContractId("XRP"),
      symbol: "XRP",
      ratio: 1,
    },
    {
      asset: toolkit.addressBook.getContractId("XLM"),
      symbol: "XLM",
      ratio: 1,
    },
  ];

  // const assetAllocations = assets.map((asset) => {
  //   return xdr.ScVal.scvMap([
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol("address"),
  //       val: asset.address.toScVal(),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol("strategies"),
  //       val: xdr.ScVal.scvVec(
  //         asset.strategies.map((strategy) =>
  //           xdr.ScVal.scvMap([
  //             new xdr.ScMapEntry({
  //               key: xdr.ScVal.scvSymbol("address"),
  //               val: new Address(strategy.address).toScVal(),
  //             }),
  //             new xdr.ScMapEntry({
  //               key: xdr.ScVal.scvSymbol("name"),
  //               val: nativeToScVal(strategy.name, { type: "string" }),
  //             }),
  //             new xdr.ScMapEntry({
  //               key: xdr.ScVal.scvSymbol("paused"),
  //               val: nativeToScVal(false, { type: "bool" }),
  //             }),
  //           ])
  //         )
  //       ),
  //     }),
  //   ]);
  // });

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
