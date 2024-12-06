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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub vault: Address,
    pub oracle: Address,
    pub asset_ratios: Vec<AssetRatio>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    pub action: ActionType,
    pub strategy: Option<Address>,
    pub amount: Option<i128>,
    pub swap_details_exact_in: OptionalSwapDetailsExactIn,
    pub swap_details_exact_out: OptionalSwapDetailsExactOut,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum ActionType {
    Withdraw = 0,
    Invest = 1,
    SwapExactIn = 2,
    SwapExactOut = 3,
    Zapper = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapDetailsExactIn {
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: i128,
    pub amount_out_min: i128,
    pub distribution: Vec<DexDistribution>,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapDetailsExactOut {
    pub token_in: Address,
    pub token_out: Address,
    pub amount_out: i128,
    pub amount_in_max: i128,
    pub distribution: Vec<DexDistribution>,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DexDistribution {
    pub protocol_id: String,
    pub path: Vec<Address>,
    pub parts: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptionalSwapDetailsExactIn {
    Some(SwapDetailsExactIn),
    None,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptionalSwapDetailsExactOut {
    Some(SwapDetailsExactOut),
    None,
}
