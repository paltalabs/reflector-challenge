use soroban_sdk::{Address, Env, Map, Vec};

use crate::model::Config;

soroban_sdk::contractimport!(file = "../defindex_vault.wasm");

pub type VaultClient<'a> = Client<'a>;

pub fn execute_rebalance(e: &Env, config: Config, instructions: Vec<Instruction>) -> () {
    let vault_client = VaultClient::new(e, &config.vault);

    vault_client.rebalance(&instructions)
}

pub fn fetch_total_managed_funds(e: &Env, config: Config) -> Map<Address, CurrentAssetInvestmentAllocation> {
    let vault_client = VaultClient::new(e, &config.vault);

    vault_client.fetch_total_managed_funds()
}
