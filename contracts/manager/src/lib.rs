#![no_std]
use crate::model::{AssetRatio};
use crate::utils::calculate_rebalance;
use crate::vault::{fetch_total_managed_funds, execute_rebalance};
use error::ContractError;
use model::Config;
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, String, Vec};

mod model;
mod oracle;
mod storage;
mod test;
mod utils;
mod vault;
mod error; 

use storage::{extend_instance_ttl, get_config, set_config};

#[contract]
pub struct TrustlessManager;

#[contractimpl]
impl TrustlessManager {
    pub fn __constructor(e: Env, vault: Address, oracle: Address, asset_ratios: Vec<AssetRatio>) {
        if asset_ratios.is_empty() {
            panic_with_error!(&e, ContractError::AssetRatiosMustNotBeEmpty);
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

    pub fn rebalance(e: Env, router: Address, pair: Address) -> String {
        extend_instance_ttl(&e);
        let config = get_config(&e);

        // Get Prices
        let asset_prices = oracle::get_prices_object(&e);
        
        // get current allocations:
        let current_allocations = fetch_total_managed_funds(&e, config.clone());
        
        // Get instructions (calculate_rebalance)
        let instructions = calculate_rebalance(
            &e,
            current_allocations,
            asset_prices,
            config.asset_ratios.clone(),
            router,
            pair);

        // Execute instructions
        
        execute_rebalance(
            &e,
            config.clone(),
            instructions
        );
        
        String::from_str(&e, "Rebalance")
    }

    // TEMP METHODS FOR TESTING
    pub fn get_prices(e: Env) -> Vec<i128> {
        let config = get_config(&e);
        let mut prices = Vec::new(&e);

        let oracle_address = config.oracle.clone();
        for asset_ratio in config.asset_ratios.iter() {
            let price = oracle::get_price(&e, oracle_address.clone(), asset_ratio.symbol);
            prices.push_back(price);
        }

        prices
    }
    
}
