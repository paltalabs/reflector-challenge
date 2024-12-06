use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetRatio {
    pub asset: Address,
    pub ratio: i128,
}
