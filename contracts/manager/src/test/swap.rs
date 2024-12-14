// Testing that the test is correctly seted up XD
use soroban_sdk::{
    testutils::Address as _, 
    vec as sorobanvec, Address, String, Vec, 
    // Address, 
    // Env, 
    // String, 
    // Val, 
    // Vec, 
    // BytesN
};
use crate::test::{TrustlessManagerTest, Asset, ConfigData};

use super::defindex_vault::{ActionType, AssetInvestmentAllocation, DexDistribution, Instruction, OptionalSwapDetailsExactIn, OptionalSwapDetailsExactOut, StrategyAllocation, SwapDetailsExactIn};

#[test]
fn test_swap() {
    let test = TrustlessManagerTest::setup();
    test.env.mock_all_auths();

    let user = Address::generate(&test.env);
    // Mint tokens to user
    let amount_0 = 1000_0_000_000;
    let amount_1 = 200_0_000_000;
    test.token_0_admin_client.mint(&user, &amount_0);
    test.token_1_admin_client.mint(&user, &amount_1);

    // Deposit to vault
    test.defindex_vault.deposit(
        &sorobanvec![&test.env, amount_0, amount_1],
        &sorobanvec![&test.env, amount_0, amount_1],
        &user,
        &true,
    );
    
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path,
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    test.token_0_admin_client.mint(&test.defindex_vault.address.clone(), &100000000);

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token_0.address.clone()),
            amount: Some(1000),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
        Instruction {
            action: ActionType::SwapExactIn,
            strategy: None,
            amount: None,
            swap_details_exact_in: OptionalSwapDetailsExactIn::Some(SwapDetailsExactIn {
                token_in: test.token_0.address.clone(),
                token_out: test.token_1.address.clone(),
                amount_in: 100,
                amount_out_min: 0,
                distribution: distribution_vec,
                deadline: test.env.ledger().timestamp() + 3600u64,
                router: test.soroswap_router.address.clone(),
                pair: test.soroswap_pair.clone(),
            }),
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
        Instruction {
            action: ActionType::Invest,
            strategy: Some(test.strategy_client_token_1.address.clone()),
            amount: Some(17),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        }
    ];

    test.defindex_vault.rebalance(&instructions);
    
}