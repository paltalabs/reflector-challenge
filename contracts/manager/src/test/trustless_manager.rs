use soroban_sdk::{
    vec as sorobanvec,
    Symbol,
};
use crate::test::{TrustlessManagerTest, Asset, ConfigData, PriceData};
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

    let token_0_price = 5000000;
    let token_1_price = 25000000;

    let ledger_info = test.env.ledger().get();
    test.env.ledger().set(LedgerInfo {
        timestamp: 900,
        ..ledger_info
    });

    // let timestamp = 600_000;
    // test.reflector.set_price(
    //     &sorobanvec![
    //         &test.env, 
    //         normalize_price(token_0_price), 
    //         normalize_price(token_1_price)], 
    //     &timestamp); //milisegundos

    // let amount_0 = 1000;
    // let amount_1 = 200;
    // test.defindex_contract.deposit(
    //     &sorobanvec![&test.env, amount_0, amount_1], // asset 0
    //     &sorobanvec![&test.env, amount_0, amount_1], // asset 1 
    //     &test.users[0],
    //     &true,
    // );

    // let balance_0 = test.token_0.balance(&test.users[0]);
    // let balance_1 = test.token_1.balance(&test.users[0]);
    // assert_eq!(balance_0, amount_0);
    // assert_eq!(balance_1, amount_1);

    // let total_managed_funds = test.defindex_contract.total_managed_funds();
    // assert_eq!(total_managed_funds, 500 + 500);

    // let token_0_price = 2000;
    // let token_1_price = 2400;

    // let timestamp = 900_000;
    // test.reflector.set_price(
    //     &sorobanvec![
    //         &test.env, 
    //         normalize_price(token_0_price), 
    //         normalize_price(token_1_price)], 
    //     &timestamp); //milisegundos

    // let rebalance = test.trustless_manager.rebalance();
    // assert_eq!(rebalance, "Rebalance");

    // let total_managed_funds = test.defindex_contract.total_managed_funds();
    // assert_eq!(total_managed_funds, 2480 + 2480);
    // let balance_0 = test.token_0.balance(&test.users[0]);
    // let balance_1 = test.token_1.balance(&test.users[0]);
    // assert_eq!(balance_0, 1240);
    // assert_eq!(balance_1, 600
}