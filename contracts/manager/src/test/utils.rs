use crate::model::Config;
use crate::vault::{CurrentAssetInvestmentAllocation, Instruction, StrategyAllocation};
use soroban_sdk::{testutils::Address as _, Address, Env, Map, Vec};

extern crate std;
use std::println;

// Import the function directly from the parent module
use super::super::utils::calculate_rebalance;

#[test]
fn test_calculate_rebalance_empty_allocations() {
    let env = Env::default();
    let current_allocations: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(&env);

    let instructions = calculate_rebalance(&env, current_allocations);
    assert_eq!(instructions.len(), 0);
}

#[test]
fn test_calculate_rebalance_multiple_assets() {
    let env = Env::default();
    let mut current_allocations: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(&env);

    // Create two assets and their strategies
    let asset1 = Address::generate(&env);
    let asset2 = Address::generate(&env);
    let strategy1 = Address::generate(&env);
    let strategy2 = Address::generate(&env);

    println!("asset1: {:?}", asset1);
    println!("asset2: {:?}", asset2);
    println!("strategy1: {:?}", strategy1);
    println!("strategy2: {:?}", strategy2);
    // Create strategy allocations for both assets
    let strategy_allocations1 = Vec::from_array(
        &env,
        [StrategyAllocation {
            strategy_address: strategy1.clone(),
            amount: 1000,
        }],
    );

    let strategy_allocations2 = Vec::from_array(
        &env,
        [StrategyAllocation {
            strategy_address: strategy2.clone(),
            amount: 2000,
        }],
    );

    // Create current allocations for both assets
    let allocation1 = CurrentAssetInvestmentAllocation {
        asset: asset1.clone(),
        total_amount: 1000,
        idle_amount: 1000,
        invested_amount: 0,
        strategy_allocations: strategy_allocations1,
    };

    let allocation2 = CurrentAssetInvestmentAllocation {
        asset: asset2.clone(),
        total_amount: 2000,
        idle_amount: 2000,
        invested_amount: 0,
        strategy_allocations: strategy_allocations2,
    };

    current_allocations.set(asset1, allocation1);
    current_allocations.set(asset2, allocation2);

    let instructions = calculate_rebalance(&env, current_allocations);
    assert_eq!(instructions.len(), 0); // For now, since implementation returns empty vec
}
