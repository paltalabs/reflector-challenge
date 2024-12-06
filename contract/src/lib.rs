#![no_std]
use crate::model::AssetRatio;
use soroban_sdk::{contract, contractimpl, Env, String, Vec};

mod model;
mod storage;

use storage::{extend_instance_ttl, set_initialized};

#[contract]
struct ReflectoyChallenge;

#[contractimpl]
impl ReflectoyChallenge {
    fn __constructor(e: Env, asset_ratios: Vec<AssetRatio>) {
        set_initialized(&e);
    }

    fn hello_world(e: Env) -> String {
        extend_instance_ttl(&e);

        String::from_str(&e, "Hello world")
    }

    fn rebalance(e: Env) {
        extend_instance_ttl(&e);
        String::from_str(&e, "Rebalance")
    }
}
