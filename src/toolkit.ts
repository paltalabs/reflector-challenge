import "dotenv/config";
import { createToolkit } from "soroban-toolkit";

export const toolkitLoader = createToolkit({
  adminSecret: process.env.ADMIN_SECRET_KEY!,
  contractPaths: {
    hodl_strategy: "./wasm/hodl_strategy.wasm",
    vault: "./wasm/defindex_vault.wasm",
    factory: "./wasm/factory.wasm",
  },
  customNetworks: [],
  verbose: "full",
});
