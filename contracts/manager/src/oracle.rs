use soroban_sdk::{panic_with_error, Address, Env, Symbol, Vec};

use crate::{error::ContractError, model::AssetRatio, model::AssetPrice, storage::get_config};

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
            panic_with_error!(e, ContractError::GetPriceError);
        }
        Some(price) => {
            price.price
        }
        
    }    
}

pub fn get_prices_object(e: &Env) -> Vec<AssetPrice> {
    let config = get_config(&e);
    let mut prices = Vec::new(&e);

    let oracle_address = config.oracle.clone();
    for asset_ratio in config.asset_ratios.iter() {
        let price_value = get_price(&e, oracle_address.clone(), asset_ratio.symbol.clone());
        let asset_price = AssetPrice {
            asset: asset_ratio.asset.clone(),
            symbol: asset_ratio.symbol.clone(),
            price: price_value,
        };
        prices.push_back(asset_price);
    }

    prices
}

