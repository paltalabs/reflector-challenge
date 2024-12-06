#![no_std]
use crate::model::AssetRatio;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod model;
mod storage;
mod oracle;

use storage::{extend_instance_ttl, get_asset_ratios, set_asset_ratios, set_vault};

#[contract]
pub struct ReflectorChallenge;

#[contractimpl]
impl ReflectorChallenge {
    pub fn __constructor(e: Env, vault: Address, asset_ratios: Vec<AssetRatio>) {
        if asset_ratios.len() == 0 {
            panic!("Asset ratios must not be empty");
        }

        set_asset_ratios(&e, asset_ratios);
        set_vault(&e, vault);
    }

    pub fn assets(e: Env) -> Vec<AssetRatio> {
        get_asset_ratios(&e)
    }

    pub fn rebalance(e: Env) -> String {
        extend_instance_ttl(&e);
        String::from_str(&e, "Rebalance")
    }
}
