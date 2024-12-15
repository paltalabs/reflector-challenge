use soroban_sdk::{contracttype, Address, Symbol, Vec, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetRatio {
    pub asset: Address,
    pub symbol: Symbol,
    pub ratio: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetPrice {
    pub asset: Address,
    pub symbol: Symbol,
    pub price: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub vault: Address,
    pub oracle: Address,
    pub asset_ratios: Vec<AssetRatio>,
    pub router: Address,
    pub pair: Address,
}
