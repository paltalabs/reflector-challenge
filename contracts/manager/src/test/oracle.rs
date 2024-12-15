use soroban_sdk::{
    // testutils::Address as _, 
    vec as sorobanvec,
    Symbol,
    // Address, 
    // Env, 
    // String, 
    // Val, 
    // Vec, 
    // BytesN
};
use crate::test::{TrustlessManagerTest, Asset, PriceData, normalize_price, convert_to_seconds};
use soroban_sdk::{testutils::{Ledger, LedgerInfo}};



// TEST THAT WE CAN SET PRICES IN THE ORACLE AND THAT WE CAN READ THEM ON THE TRUSTLESS MANAGER
#[test]
fn test_set_prices() {
    let test = TrustlessManagerTest::setup();

    let assets = test.reflector.assets();
    let expected_assets = sorobanvec![
        &test.env,
        Asset::Other(Symbol::new(&test.env, "XLM")),
        Asset::Other(Symbol::new(&test.env, "XRP")),
    ];
    assert_eq!(assets, expected_assets);

    let token_0_price = 1000;
    let token_1_price = 2000;

    let ledger_info = test.env.ledger().get();
    test.env.ledger().set(LedgerInfo {
        timestamp: 900,
        ..ledger_info
    });
    
    
    // pub fn set_price(e: Env, updates: Vec<i128>, timestamp: u64) {
    let timestamp = 600_000;
    test.reflector.set_price(
        &sorobanvec![
            &test.env, 
            normalize_price(token_0_price), 
            normalize_price(token_1_price)], 
        &timestamp); //milisegundos

    let timestamp = 900_000;
    test.reflector.set_price(
        &sorobanvec![
            &test.env, 
            normalize_price(token_0_price), 
            normalize_price(token_1_price)], 
        &timestamp);
    
   
    let last_price_xlm = test.reflector.lastprice(
        &Asset::Other(Symbol::new(&test.env, "XLM")));
    assert_ne!(last_price_xlm, None);
    assert_eq!(
        last_price_xlm,
        Some(PriceData {
            price: normalize_price(token_0_price),
            timestamp: convert_to_seconds(900_000),
        })
    );
    
    // get prices from trustless manager
    // pub fn get_prices(e: Env) -> Vec<i128> {
    let prices = test.trustless_manager.get_prices();
    assert_eq!(prices, 
        sorobanvec![&test.env, 
        normalize_price(token_0_price), 
        normalize_price(token_1_price)]);

}