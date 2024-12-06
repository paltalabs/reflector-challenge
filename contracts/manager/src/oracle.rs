use soroban_sdk::{Env, Symbol};

use crate::{model::AssetRatio, storage::get_config};

soroban_sdk::contractimport!(
    file = "src/reflector.wasm"
);

pub type OracleClient<'a> = Client<'a>;

pub fn get_price(e: &Env, asset: &AssetRatio) -> i128 {
    let config = get_config(e);
    let oracle = config.oracle;

    let oracleClient = OracleClient::new(&e, &oracle);
    
    if asset.symbol == Symbol::new(e, "XRP") {

        i128::from(2_2_698_160) // Price for XRP
    } else if asset.symbol == Symbol::new(e, "XLM") {
        i128::from(4_578_725) // Price for XLM
    } else {
        panic!("Unsupported asset symbol")
    }
}
