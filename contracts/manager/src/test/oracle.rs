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
use crate::test::{TrustlessManagerTest, Asset, ConfigData, PriceData};
use soroban_sdk::{testutils::{Ledger, LedgerInfo}};

const DECIMALS: u32 = 14;
fn normalize_price(price: i128) -> i128 {
    price * 10i128.pow(DECIMALS)
}
fn convert_to_seconds(timestamp: u64) -> u64 {
    timestamp / 1000
}

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
        &timestamp);

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
    // // pub struct PriceData {
    //     // The price in contracts' base asset and decimals.
    //     pub price: i128,
    //     // The timestamp of the price.
    //     pub timestamp: u64,
    // }
    
//     let expected_price_data = PriceData {
//         price: token_0_price,
//         timestamp: current_time,
//     };
//     // assert_eq!(last_price_xlm, Some(expected_price_data));


//     // tes price at specific timestamp
//     // pub fn price(e: Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
//     let price_xlm = test.reflector.price(&Asset::Other(Symbol::new(&test.env, "XLM")), &current_time);
//     assert_eq!(price_xlm, Some(expected_price_data));
// // 



    
    
    // .prices(&Asset::Stellar(test.token_0.address), &1u32);
    // prices is some
    // assert_eq!(prices_is, token_0_price);

    // let prices = test.trustless_manager.get_prices();
    // assert_eq!(prices.len(), 2);
    // assert_eq!(prices.get(0).unwrap(), token_0_price);
    // assert_eq!(prices[1], token_1_price);
}