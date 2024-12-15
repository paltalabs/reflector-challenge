import "dotenv/config";
import {
  deploySorobanToken,
  invokeContract,
  invokeCustomContract,
} from "soroban-toolkit";
import { toolkitLoader } from "./toolkit";
import { Address, Keypair, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import fs from "fs";

async function main() {
  const toolkit = toolkitLoader.getNetworkToolkit("testnet");
  // The contractKey for the XRP Token is "soroban_token"
  // params is params: xdr.ScVal[],
  // mintParams [from, to, amount]

  // will mint 1,000 units of XRP = 1,000.0000000 = 10000000000
  const mintTo = new Address(toolkit.admin.publicKey()).toScVal();
  const mintAmount = nativeToScVal(10000000000, { type: "i128" });
  const mintParams: xdr.ScVal[] = [mintTo, mintAmount];

  await invokeContract(toolkit, "XRP", "mint", mintParams);

  // at the time of writing
  // 1 XRP = $2.39 ==> 1 USD = 0.418410042 XRP
  // 1 XLM = $0.41 ==> 1 USD = 2.43902439 XLM

  // We will create a 2000 USD Valued LP, with 1000 USD of XRP and 1000 USD of XLM
  // 1000 USD of XLM = 1000 * 2.43902439 = 2439.0243900 XLM
  // 1000 USD of XRP = 1000 * 0.418410042 = 418.4100420 XRP

  // Get Soroswap Router Address from public/testnet.soroswap.json using fs.readFileSync

  const network = "testnet";
  // should use fs.readFileSync to get the router address from thje json
  const addressesJson = fs
    .readFileSync(`./public/${network}.contracts.json`)
    .toString();
  const addresses = JSON.parse(addressesJson);
  const routerAddress = addresses.soroswap_router;
  console.log("routerAddress", routerAddress);

  //   fn add_liquidity(
  //     e: Env,
  //     token_a: Address,
  //     token_b: Address,
  //     amount_a_desired: i128,
  //     amount_b_desired: i128,
  //     amount_a_min: i128,
  //     amount_b_min: i128,
  //     to: Address,
  //     deadline: u64,
  // ) -> Result<(i128, i128, i128), CombinedRouterError>;

  // xlm address is in public

  const timestamp2030 = 1922796601;

  // Add liquidity
  const addLiquidityScValParams: xdr.ScVal[] = [
    new Address(addresses.xlm).toScVal(),
    new Address(toolkit.addressBook.getContractId("XRP")).toScVal(),
    nativeToScVal(2439_0_243_900, { type: "i128" }),
    nativeToScVal(418_4_100_420, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    nativeToScVal(0, { type: "i128" }),
    new Address(toolkit.admin.publicKey()).toScVal(),
    nativeToScVal(timestamp2030, { type: "u64" }),
  ];
  invokeCustomContract(
    toolkit,
    routerAddress,
    "add_liquidity",
    addLiquidityScValParams
  );
}

main();
