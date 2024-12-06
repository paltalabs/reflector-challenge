use crate::model::AssetRatio;
use soroban_sdk::{contractimport, Address, Env};

pub fn get_price(e: &Env, asset: &AssetRatio) -> i128 {
    if asset.symbol == Symbol::new(e, "XRP") {
        i128::from(2_2_698_160) // Price for XRP
    } else if asset.symbol == Symbol::new(e, "XLM") {
        i128::from(4_578_725) // Price for XLM
    } else {
        panic!("Unsupported asset symbol")
    }
}
