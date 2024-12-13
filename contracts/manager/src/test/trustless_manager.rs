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
fn () {
    todo!()
}