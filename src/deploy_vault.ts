import {
  Address,
  Asset,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { toolkitLoader } from "./toolkit";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
} from "soroban-toolkit";
import { randomBytes } from "crypto";

const toolkit = toolkitLoader.getNetworkToolkit("testnet");

export async function deployVaults() {
  await airdropAccount(toolkit, toolkit.admin);
  let account = await toolkit.horizonRpc.loadAccount(toolkit.admin.publicKey());
  console.log("publicKey", toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying vault");
  console.log("-------------------------------------------------------");
  await installContract(toolkit, "vault");
  const assets = [
    {
      address: toolkit.addressBook.getContractId("XRP"),
      strategies: [
        {
          name: "Hodl XRP",
          address: toolkit.addressBook.getContractId("hodl_xrp"),
          paused: false,
        },
      ],
    },
    {
      address: Asset.native().contractId(toolkit.passphrase),
      strategies: [
        {
          name: "Hodl XLM",
          address: toolkit.addressBook.getContractId("hodl_xlm"),
          paused: false,
        },
      ],
    },
  ];

  const assetAllocations = assets.map((asset) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("address"),
        val: new Address(asset.address).toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("strategies"),
        val: xdr.ScVal.scvVec(
          asset.strategies.map((strategy) =>
            xdr.ScVal.scvMap([
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("address"),
                val: new Address(strategy.address).toScVal(),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("name"),
                val: nativeToScVal(strategy.name, { type: "string" }),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("paused"),
                val: nativeToScVal(false, { type: "bool" }),
              }),
            ])
          )
        ),
      }),
    ]);
  });

  /* 
  fn create_defindex_vault(
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_fee: u32,
        vault_name: String,
        vault_symbol: String,
        manager: Address,
        assets: Vec<AssetStrategySet>,
        salt: BytesN<32>
    ) 
  */

  const createDeFindexParams: xdr.ScVal[] = [
    new Address(toolkit.admin.publicKey()).toScVal(), // emergency_manager
    new Address(toolkit.admin.publicKey()).toScVal(), // fee_receiver
    nativeToScVal(100, { type: "u32" }), // vault_fee
    nativeToScVal("AAA/XRP", { type: "string" }), // vault_name
    nativeToScVal("HXRM", { type: "string" }), // vault_symbol
    new Address(toolkit.admin.publicKey()).toScVal(), // manager
    xdr.ScVal.scvVec(assetAllocations), // assets
    nativeToScVal(randomBytes(32)), // salt
  ];

  const result = await invokeContract(
    toolkit,
    "defindex_factory",
    "create_defindex_vault",
    createDeFindexParams
  );

  console.log(
    "🚀 « DeFindex Vault created with address:",
    scValToNative(result.returnValue)
  );

  toolkit.addressBook.setContractId("vault", scValToNative(result.returnValue));
  toolkit.addressBook.writeToFile();

  return scValToNative(result.returnValue);
}

async function main() {
  try {
    await deployVaults();
    toolkit.addressBook.writeToFile();
  } catch (e) {
    console.error(e);
  }
}

main();
