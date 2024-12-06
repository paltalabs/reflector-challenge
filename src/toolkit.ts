import "dotenv/config";
import { createToolkit } from "soroban-toolkit";

export const toolkitLoader = createToolkit({
  adminSecret: process.env.ADMIN_SECRET_KEY!,
  contractPaths: {
    hodl_strategy: "./contracts/hodl_strategy.wasm",
    vault: "./contracts/defindex_vault.wasm",
    factory: "./contracts/factory.wasm",
  },
  customNetworks: [],
  verbose: "full",
});
