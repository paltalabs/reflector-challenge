import "dotenv/config";
import { createToolkit } from "soroban-toolkit";

export const toolkitLoader = createToolkit({
  adminSecret: process.env.ADMIN_SECRET_KEY!,
  contractPaths: {
    hodl_strategy: "./contracts/hodl_strategy.wasm",
    vault: "./contracts/defindex_vault.wasm",
    factory: "./contracts/defindex_factory.wasm",
    tmanager:
      "./contracts/manager/target/wasm32-unknown-unknown/release/trusless_manager.optimized.wasm",
  },
  customNetworks: [],
  verbose: "full",
});
