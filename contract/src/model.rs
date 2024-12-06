use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetRatio {
    pub asset: Address,
    pub symbol: Symbol,
    pub ratio: i128,
}
