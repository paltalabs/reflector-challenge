import { Address, Asset, nativeToScVal, xdr } from "@stellar/stellar-sdk";
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

  const assetRatiosRaw = [
    {
      asset: toolkit.addressBook.getContractId("XRP"),
      symbol: "XRP",
      ratio: 1,
    },
    {
      asset: Asset.native().contractId(toolkit.passphrase),
      symbol: "XLM",
      ratio: 1,
    },
  ];

  const assetRatios = assetRatiosRaw.map((asset) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("asset"),
        val: new Address(asset.asset).toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("ratio"),
        val: nativeToScVal(asset.ratio, { type: "i128" }),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("symbol"),
        val: nativeToScVal(asset.symbol, { type: "string" }),
      }),
    ]);
  });

  await installContract(toolkit, "tmanager");
  await deployContract(toolkit, "tmanager", "tmanager", [
    new Address(toolkit.addressBook.getContractId("vault")).toScVal(),
    new Address(
      "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63"
    ).toScVal(),
    xdr.ScVal.scvVec(assetRatios),
  ]);
}

deployTManager();
