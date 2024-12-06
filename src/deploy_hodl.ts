import { Address, Asset, nativeToScVal, Networks, xdr } from "@stellar/stellar-sdk";
import { toolkitLoader } from "./toolkit";
import { airdropAccount, deployContract, installContract, invokeContract,  } from "soroban-toolkit";


const toolkit = toolkitLoader.getNetworkToolkit("testnet");
export async function deployContracts() {
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
  await deployContract(
    toolkit,
    "hodl_strategy",
    [nativeToScVal("hodl_strategy")],
    toolkit.admin,
  );

  const xlm = Asset.native();
  let xlmContractId: string;
  switch (network) {
    case "testnet":
      xlmContractId = xlm.contractId(Networks.TESTNET);
      break;
    case "mainnet":
      xlmContractId = xlm.contractId(Networks.PUBLIC);
      break;
    default:
      console.log("Invalid network:", network, "It should be either testnet or mainnet");
      return;
      break;
  }
  const xlmAddress = new Address(xlmContractId);
  const xlmScVal = xlmAddress.toScVal();

  const soroswapUSDC = new Address("CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI");
  const usdcScVal = soroswapUSDC.toScVal();

  const emptyVecScVal = xdr.ScVal.scvVec([]);

  console.log("Initializing DeFindex HODL Strategy");
  await invokeContract(
    toolkit,
    "hodl_strategy",
    "initialize",
    [usdcScVal],
    false,
    toolkit.admin
  );
}

const network = process.argv[2];

async function main(){
  try {
    await deployContracts();
    toolkit.addressBook.writeToFile();
  } catch (e) {
    console.error(e);
  }
}

main();
