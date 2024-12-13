use soroban_sdk::{
    vec as sorobanvec,
    Symbol,
    Map,
};
use crate::test::{
    TrustlessManagerTest, 
    Asset, 
    ConfigData, 
    PriceData,
    CurrentAssetInvestmentAllocation,
    StrategyAllocation,
    normalize_price, convert_to_seconds};
use soroban_sdk::{testutils::{Ledger, LedgerInfo}};

/*
- we need to set up an original prices for XLM and XRP
- XLM is 0.5 USD, XRP is 2.5 USD

- user needs to deposit tokens into the vault, check vault deposit tests in defindex
- if user deposits AND INVEST 1000 XLM, => 500 USD
- user needs to deposit 200 XRP, => 500 USD
- now assets will be in idle funds
 defindex_contract.deposit(
        &sorobanvec![&test.env, amount_0, amount_1], // asset 0
        &sorobanvec![&test.env, amount_0, amount_1], // asset 1 
        &users[0],
        &rue,
    );



- check that the vault has the new assets
// check total managed funds
----------------

- set new prices for XLM and XRP
- XLM is 2 USD, XRP is 2.4 USD
 supposing that we have 2000 XLM and 400 XRP
 the new total managed funds will be 4000 USD + 960 USD = 4960 USD,
 that for a 50% 50% vault should be distribued as
    2480 USD in XLM and 2480 USD in XRP
    1240 XLM and 2480/400 = 600 XRP

// we ask the trusless manager to balance the vault

now we check the new total managed funds that should be correct



*/
#[test]
fn trustless_manager_works() {
    let test = TrustlessManagerTest::setup();

    // We set an original price for each token
    // XLM is 0.5 USD, XRP is 2.5 USD
    let xlm_price = 5000000;
    let xrp_price = 25000000;

    let ledger_info = test.env.ledger().get();
    test.env.ledger().set(LedgerInfo {
        timestamp: 900,
        ..ledger_info
    });

    let timestamp = 600_000;
    test.reflector.set_price(
        &sorobanvec![
            &test.env, 
            normalize_price(xlm_price), 
            normalize_price(xrp_price)], 
        &timestamp); //milisegundos

    // A user wants to invest 500 USD worth of XLM and 500 USD worth of XRP
    // We need to deposit 1000 XLM and 200 XRP
    let deposit_amount_xlm = 1000_0_000_000i128;
    let deposit_amount_xrp = 200_0_000_000i128;
    test.token_0_admin_client.mint(&test.user, &deposit_amount_xlm);
    test.token_1_admin_client.mint(&test.user, &deposit_amount_xrp);

    test.defindex_vault.deposit(
        &sorobanvec![&test.env, deposit_amount_xlm, deposit_amount_xrp], // asset 0
        &sorobanvec![&test.env, deposit_amount_xlm, deposit_amount_xrp], // asset 1 
        &test.user,
        &true,
    );

    // // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_0.address.clone(),
        amount: 2000_0_000_000i128,
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_1.address.clone(),
        amount: 400_0_000_000i128,
    }];
    total_managed_funds_expected.set(test.token_0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        total_amount: 2000_0_000_000i128,
        idle_amount: 0,
        invested_amount: 2000_0_000_000i128,
        strategy_allocations: strategy_investments_expected_token_0,
    });
    total_managed_funds_expected.set(test.token_1.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        total_amount: 400_0_000_000i128,
        idle_amount: 0,
        invested_amount: 400_0_000_000i128,
        strategy_allocations: strategy_investments_expected_token_1,
    });

    let total_managed_funds = test.defindex_vault.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
    

    // 3 DORITOS LATER
    let ledger_info = test.env.ledger().get();
    test.env.ledger().set(LedgerInfo {
        timestamp: 1000,
        ..ledger_info
    });

    // NOW THE ORACLE CHANGED ITS VALUE
    // NOW XLM PUMPED X 10 ! Oh my god!
    // XRP pumped only x 2
    // so now they have the same price!

    let xlm_price = 50000000;
    let xrp_price = 50000000;
    
    let timestamp = 1000_000;
    test.reflector.set_price(
        &sorobanvec![
            &test.env, 
            normalize_price(xlm_price), 
            normalize_price(xrp_price)], 
        &timestamp); 

    // we check the new prices
    let last_price_xlm = test.reflector.lastprice(
        &Asset::Other(Symbol::new(&test.env, "XLM")));
    assert_ne!(last_price_xlm, None);
    assert_eq!(
        last_price_xlm,
        Some(PriceData {
            price: normalize_price(xlm_price),
            timestamp: convert_to_seconds(1000_000),
        })
    );
    let last_price_xrp = test.reflector.lastprice(
        &Asset::Other(Symbol::new(&test.env, "XRP")));
    assert_ne!(last_price_xrp, None);
    assert_eq!(
        last_price_xrp,
        Some(PriceData {
            price: normalize_price(xrp_price),
            timestamp: convert_to_seconds(1000_000),
        })
    );
    
    /*
        The vault had 2,000*0.5=1,000 USD in XLM and 400*2.5= 1,000 USD in XRP
        Now the vault has 
        2000*5=10,000 USD in XLM
        400*5= 2,000 USD in XRP
        Total 12,000 USD.
        This should be splited in 6,000 USD in XLM and 6,000 USD in XRP
        6,000/5=1,200 XLM
        6,000/5=1,200 XRP
    */

    // THE MAGIC MOMENT, REBALANCING USING TRUSTLESS MANAGER
    let rebalance = test.trustless_manager.rebalance();

    // check the new total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_0.address.clone(),
        amount: 6000_0_000_000i128,
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_1.address.clone(),
        amount: 6000_0_000_000i128,
    }];
    total_managed_funds_expected.set(test.token_0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        total_amount: 6000_0_000_000i128,
        idle_amount: 0,
        invested_amount: 6000_0_000_000i128,
        strategy_allocations: strategy_investments_expected_token_0,
    });
    total_managed_funds_expected.set(test.token_1.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        total_amount: 6000_0_000_000i128,
        idle_amount: 0,
        invested_amount: 6000_0_000_000i128,
        strategy_allocations: strategy_investments_expected_token_1,
    });

    let total_managed_funds = test.defindex_vault.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

}