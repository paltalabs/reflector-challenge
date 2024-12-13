use soroban_sdk::{contract, contractimpl, Address, Env, Map, Vec};

use crate::model::Config;

soroban_sdk::contractimport!(file = "../defindex_vault.wasm");

pub type VaultClient<'a> = Client<'a>;

pub fn execute_rebalance(e: &Env, config: Config, instructions: Vec<Instruction>) -> () {
    let vault_client = VaultClient::new(e, &config.vault);

    vault_client.rebalance(&instructions)
}
