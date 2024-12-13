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

const network = process.argv[2];

const toolkit = toolkitLoader.getNetworkToolkit("testnet");

export async function deployVaults() {
  if (network != "mainnet") await airdropAccount(toolkit, toolkit.admin);
  let account = await toolkit.horizonRpc.loadAccount(toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying vault");
  console.log("-------------------------------------------------------");

  const assets = [
    {
      address: Asset.native().contractId(toolkit.passphrase),
      strategies: [
        {
          name: "Hodl XLM Strategy",
          address: toolkit.addressBook.getContractId("hodl_xlm"),
          paused: false
        },
      ]
    },
    {
      address: toolkit.addressBook.getContractId("XRP"),
      strategies: [
        {
          name: "Hodl XRP Strategy",
          address: toolkit.addressBook.getContractId("hodl_xrp"),
          paused: false
        }
      ]
    },
  ];

  const assetAllocations = assets.map((asset) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("address"),
        val: nativeToScVal(asset.address),
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

  const createDeFindexParams: xdr.ScVal[] = [
    new Address(toolkit.admin.publicKey()).toScVal(),   // emergency_manager
    new Address(toolkit.admin.publicKey()).toScVal(),   // fee_receiver
    nativeToScVal(100, { type: "u32" }),                // vault_fee
    nativeToScVal("SampleVault", { type: "string" }),   // vault_name
    nativeToScVal("HXRM", { type: "string" }),          // vault_symbol
    new Address(toolkit.admin.publicKey()).toScVal(),   // manager
    xdr.ScVal.scvVec(assetAllocations),                 // assets
    nativeToScVal(randomBytes(32)),                     // salt
  ];

  const createDefindexParamsScvec = xdr.ScVal.scvVec(createDeFindexParams);

  const result = await invokeContract(
    toolkit,
    "defindex_factory",
    "create_defindex_vault",
    [createDefindexParamsScvec],
    false,
    toolkit.admin
  );


  console.log('🚀 « DeFindex Vault created with address:', scValToNative(result.returnValue));
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


/*

    '   1: [Diagnostic Event] topics:[fn_call, CCI7MPZKUBRSJ5RD7WS77BM76R34Y2BADA6PW3GVEREPXMCXEGZ65UV2, create_defindex_vault], 
    data:[
      [{
        address: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", 
        strategies: [{ 
            address: CCTDAEPPD34ZEFVFCJ5SXWL3Q2JFWTMY53SEUN5HNAE56JEHESV2CF7C, 
            name: "Hodl XLM Strategy", 
            paused: false
          }]
        }, {
            address: "CCFDTMC25A4VGDZVJC5T4XLUTYICDBVKUVRMZJVPBZMKNTV2IQHHMVQO", 
            strategies: [{address: CDZIA5B7YI2RHHILDT3VSWEJBOND655MXNWNB4YBXCWOHRCLSZV3FUAM, 
            name: "Hodl XRP Strategy", 
            paused: false
          }]
        }], 
        GAGZPYKRLBXHN3O2NAWAVOP6MWIPJSQAQO3NQ42H54ZBMNF73E4YOH2E, 
        GAGZPYKRLBXHN3O2NAWAVOP6MWIPJSQAQO3NQ42H54ZBMNF73E4YOH2E, 
        100, 
        "SampleVault", 
        "HXRM", 
        GAGZPYKRLBXHN3O2NAWAVOP6MWIPJSQAQO3NQ42H54ZBMNF73E4YOH2E, 
        Bytes(b0d3a107e5d3407bea50c32573b1fefa02194cf703a726573df07144ee137683)]
  */