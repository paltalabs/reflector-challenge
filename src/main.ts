import "dotenv/config";
import { toolkitLoader } from "./toolkit";

async function main() {
  const toolkit = toolkitLoader.getNetworkToolkit("testnet");

  console.log(toolkit);
}

main();