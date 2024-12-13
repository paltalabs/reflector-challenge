// Testing that the test is correctly seted up XD
use soroban_sdk::{
    // testutils::Address as _, 
    vec as sorobanvec, 
    Symbol,
    Map,
    // Address, 
    // Env, 
    // String, 
    // Val, 
    // Vec, 
    // BytesN
};
use crate::test::{TrustlessManagerTest, Asset, ConfigData, StrategyAllocation};

#[test]
fn test_setup() {
    let test = TrustlessManagerTest::setup();

    let factory_admin = test.defindex_factory.admin();
    let factory_defindex_receiver = test.defindex_factory.defindex_receiver();
  
    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);

    // Mint tokens to user
    let amount = 987654321;
    test.token_0_admin_client.mint(&test.user, &amount);
    test.token_1_admin_client.mint(&test.user, &amount);

    // check balances
    let balance_0 = test.token_0.balance(&test.user);
    let balance_1 = test.token_1.balance(&test.user);
    assert_eq!(balance_0, amount);
    assert_eq!(balance_1, amount);

    // check hodl strategy correclty set up
    assert_eq!(test.strategy_client_token_0.asset(), test.token_0.address);
    assert_eq!(test.strategy_client_token_1.asset(), test.token_1.address);

    // check expected assets from reflector
    let assets = test.reflector.assets();
    let expected_assets = sorobanvec![
        &test.env,
        Asset::Other(Symbol::new(&test.env, "XLM")),
        Asset::Other(Symbol::new(&test.env, "XRP")),
    ];
    assert_eq!(assets, expected_assets);

    // check that the vault manager is the trustless manager contract
    let vault_manager = test.defindex_vault.get_manager();
    assert_eq!(vault_manager, test.trustless_manager.address);

    let price_quote = test.soroswap_router.get_amounts_out(&test.soroswap_factory.address, &1000_0_000_000i128, &sorobanvec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()]);
    assert_eq!(price_quote, sorobanvec![&test.env, 1000_0_000_000i128, 161_9_948_741]);

    // check total managed funds
    // let mut total_managed_funds_expected = Map::new(&test.env);
    // let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
    //     strategy_address: test.strategy_client_token_0.address.clone(),
    //     amount: 1000,
    // }];
    // let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
    //     strategy_address: test.strategy_client_token1.address.clone(),
    //     amount: 200,
    // }];
    // total_managed_funds_expected.set(test.token0.address.clone(),
    // CurrentAssetInvestmentAllocation {
    //     asset: test.token0.address.clone(),
    //     total_amount: amount_0,
    //     idle_amount: amount_0 - 100,
    //     invested_amount: 100i128,
    //     strategy_allocations: strategy_investments_expected_token_0,
    // });
    // total_managed_funds_expected.set(test.token1.address.clone(),
    // CurrentAssetInvestmentAllocation {
    //     asset: test.token1.address.clone(),
    //     total_amount: amount_1,
    //     idle_amount: amount_1 - 200,
    //     invested_amount: 200i128,
    //     strategy_allocations: strategy_investments_expected_token_1,
    // });

    // let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    // assert_eq!(total_managed_funds, total_managed_funds_expected);
}