use soroban_sdk::{Env};

use crate::{model::AssetRatio, storage::get_config};

soroban_sdk::contractimport!(
    file = "../reflector.wasm"
);

pub type OracleClient<'a> = Client<'a>;

pub fn get_price(e: &Env, asset_ratio: &AssetRatio) -> i128 {
    let config = get_config(e);

    let oracle_client = OracleClient::new(&e, &config.oracle);

    let asset = Asset::Other(asset_ratio.symbol.clone());


    let price = oracle_client.price(&asset, &e.ledger().timestamp());

    match price {
        None => {
            panic!("Price not found for asset: {:?}", asset_ratio.symbol);
        }
        Some(price) => {
            price.price
        }
        
    }    
}
