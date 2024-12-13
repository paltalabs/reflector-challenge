#![no_std]
use crate::model::AssetRatio;
use model::Config;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod model;
mod storage;
mod oracle;
mod test;

use storage::{extend_instance_ttl, get_config, set_config};

#[contract]
pub struct TrustlessManager;

#[contractimpl]
impl TrustlessManager {
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
    pub fn get_price(e: Env) -> i128 {
        let asset_ratio = get_config(&e).asset_ratios.get(0).unwrap();
        oracle::get_price(&e, &asset_ratio)
    }
}
