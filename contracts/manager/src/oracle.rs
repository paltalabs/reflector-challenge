use soroban_sdk::{Env, Symbol};

use crate::{model::AssetRatio, storage::get_config};

soroban_sdk::contractimport!(
    file = "../reflector.wasm"
);

pub type OracleClient<'a> = Client<'a>;

pub fn get_price(e: &Env) -> i128 {
    let config = get_config(e);

    let oracle_client = OracleClient::new(&e, &config.oracle);

    let asset = Asset::Other(Symbol::new(&e, "XRP"));

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
