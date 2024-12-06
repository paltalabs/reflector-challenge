import "dotenv/config";
import { deploySorobanToken } from "soroban-toolkit";
import { toolkitLoader,  } from "./toolkit";


async function main() {
  const toolkit = toolkitLoader.getNetworkToolkit("testnet");
  const xrpContractAddress = await deploySorobanToken(toolkit, "XRP", "XRP", 7);
  console.log("xrpContractAddress", xrpContractAddress);
}

main();