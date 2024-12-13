#![no_std]
use crate::model::AssetRatio;
use model::Config;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod model;
mod oracle;
mod storage;
mod test;
mod utils;
mod vault;

use storage::{extend_instance_ttl, get_config, set_config};

#[contract]
pub struct ReflectorChallenge;

#[contractimpl]
impl ReflectorChallenge {
    pub fn __constructor(e: Env, vault: Address, oracle: Address, asset_ratios: Vec<AssetRatio>) {
        if asset_ratios.len() == 0 {
            panic!("Asset ratios must not be empty");
        }

        let config = Config {
            vault,
            oracle,
            asset_ratios,
        };

        set_config(&e, config);
    }

    pub fn config(e: Env) -> Config {
        get_config(&e)
    }

    pub fn rebalance(e: Env) -> String {
        extend_instance_ttl(&e);
        String::from_str(&e, "Rebalance")
    }

    // TEMP METHODS FOR TESTING
    pub fn get_price(e: Env) -> AssetRatio {
        let config = get_config(&e);
        config.asset_ratios.get(0).unwrap()
        // oracle::get_price(&e)
    }
}
