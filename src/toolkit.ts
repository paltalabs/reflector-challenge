import "dotenv/config";
import { createToolkit } from "soroban-toolkit";

export const toolkitLoader = createToolkit({
  adminSecret: process.env.ADMIN_SECRET_KEY!,
  contractPaths: {
    hodl_strategy: "./contracts/hodl_strategy.wasm",
    vault: "./contracts/vault.wasm",
    factory: "./contracts/factory.wasm",
    tmanager:
      "./contracts/manager/target/wasm32-unknown-unknown/release/trusless_manager.optimized.wasm",
  },
  customNetworks: [],
  verbose: "full",
});
