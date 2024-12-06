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
    // Create sample token addresses
    let token_XRP = Address::from_str(
        &env,
        "CACY3RX5UGOG43AZ5O4SVWRPPVXLTHBU3CKPFLCRPB5BY46LES6JLYOR",
    );
    let token_XLM = Address::from_str(
        &env,
        "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
    );
    let hodl_xlm = Address::from_str(
        &env,
        "CCJO5RFBQDGN3VMY6AUEGWU2G7LEKEN7TCHOJYGLA26QN55BOUAWCJV4",
    );
    let hodl_xrp = Address::from_str(
        &env,
        "CAHWETYSTW2WEW3RRSOZI6QVBY5MJX75EA3ZBGLSP5HXU2JDE23S2LTL",
    );

    println!("token_XRP: {:?}", token_XRP);
    println!("token_XLM: {:?}", token_XLM);
    println!("hodl_xrp: {:?}", hodl_xrp);
    println!("hodl_xlm: {:?}", hodl_xlm);
    // Create strategy allocations for both assets
    let strategy_allocations1 = Vec::from_array(
        &env,
        [StrategyAllocation {
            strategy_address: hodl_xrp.clone(),
            amount: 1000,
        }],
    );

    let strategy_allocations2 = Vec::from_array(
        &env,
        [StrategyAllocation {
            strategy_address: hodl_xlm.clone(),
            amount: 2000,
        }],
    );

    // Create current allocations for both assets
    let allocation1 = CurrentAssetInvestmentAllocation {
        asset: token_XRP.clone(),
        total_amount: 1000,
        idle_amount: 1000,
        invested_amount: 0,
        strategy_allocations: strategy_allocations1,
    };

    let allocation2 = CurrentAssetInvestmentAllocation {
        asset: token_XLM.clone(),
        total_amount: 2000,
        idle_amount: 2000,
        invested_amount: 0,
        strategy_allocations: strategy_allocations2,
    };

    current_allocations.set(token_XRP, allocation1);
    current_allocations.set(token_XLM, allocation2);

    let instructions = calculate_rebalance(&env, current_allocations);
    assert_eq!(instructions.len(), 0); // For now, since implementation returns empty vec
}
