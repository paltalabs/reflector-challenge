// Testing that the test is correctly seted up XD
use soroban_sdk::{
    testutils::Address as _, 
    vec as sorobanvec, Address, 
    // Address, 
    // Env, 
    // String, 
    // Val, 
    // Vec, 
    // BytesN
};
use crate::test::{TrustlessManagerTest, Asset, ConfigData};

use super::defindex_vault::{ActionType, AssetInvestmentAllocation, Instruction, StrategyAllocation};

#[test]
fn test_swap() {
    let test = TrustlessManagerTest::setup();
    test.env.mock_all_auths();

    let user = Address::generate(&test.env);
    // Mint tokens to user
    let amount_0 = 10000_0_000_000;
    let amount_1 = 1000_0_000_000;
    test.token_0_admin_client.mint(&user, &amount_0);
    test.token_1_admin_client.mint(&user, &amount_1);

    // Deposit to vault
    test.defindex_vault.deposit(
        &sorobanvec![&test.env, amount_0, amount_1],
        &sorobanvec![&test.env, amount_0, amount_1],
        &user,
        &false,
    );


    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount_0,
                }),
            ],
        }),
        Some(AssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_1.address.clone(),
                    amount: amount_1,
                }),
            ],
        }),
    ];

    test.defindex_vault.invest(&investments);

    // Rebalance from here on
    // let instructions = sorobanvec![
    //     &test.env,
    //     Instruction {
    //         action: ActionType::Withdraw,
    //         strategy: Some(test.strategy_client_token_0.address.clone()),
    //         amount: Some(1000),
    //         swap_details_exact_in: OptionalSwapDetailsExactIn::None,
    //         swap_details_exact_out: OptionalSwapDetailsExactOut::None,
    //     },
    //     Instruction {
    //         action: ActionType::SwapExactIn,
    //         strategy: None,
    //         amount: None,
    //         swap_details_exact_in: OptionalSwapDetailsExactIn::Some(SwapDetailsExactIn {
    //             token_in: test.token_0.address.clone(),
    //             token_out: test.token_1.address.clone(),
    //             amount_in: 1000,
    //             amount_out_min: 0,
    //             distribution: ,
    //             deadline: test.env.ledger().timestamp() + 3600u64,
    //         }),
    //         swap_details_exact_out: OptionalSwapDetailsExactOut::None,
    //     },
    //     Instruction {
    //         action: ActionType::Invest,
    //         strategy: Some(test.strategy_client_token_1.address.clone()),
    //         amount: Some(expected_swap_out?),
    //         swap_details_exact_in: OptionalSwapDetailsExactIn::None,
    //         swap_details_exact_out: OptionalSwapDetailsExactOut::None,
    //     }
    // ];

    // test.defindex_vault.rebalance(&instructions);
    
}