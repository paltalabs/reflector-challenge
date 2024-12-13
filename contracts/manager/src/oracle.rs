use soroban_sdk::{Env, Symbol, Address};

use crate::{model::AssetRatio, storage::get_config};

soroban_sdk::contractimport!(
    file = "../reflector.wasm"
);

pub type OracleClient<'a> = Client<'a>;

pub fn get_price(e: &Env, oracle: Address, symbol: Symbol) -> i128 {
    let oracle_client = OracleClient::new(&e, &oracle);

    let asset = Asset::Other(symbol);

    let price = oracle_client.lastprice(&asset);

    match price {
        None => {
            panic!("Price not found for asset: {:?}", "XRP");
        }
        Some(price) => {
            price.price
        }
        
    }    
}
