use soroban_sdk::{contracttype, Address, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetRatio {
    pub asset: Address,
    pub symbol: Symbol,
    pub ratio: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyAllocation {
    pub strategy_address: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentAssetInvestmentAllocation {
    pub asset: Address,
    pub total_amount: i128,
    pub idle_amount: i128,
    pub invested_amount: i128,
    pub strategy_allocations: Vec<StrategyAllocation>,
}
