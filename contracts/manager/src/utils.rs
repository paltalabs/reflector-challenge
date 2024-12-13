use crate::model::{Config,AssetPrice, AssetRatio};
use crate::vault::{CurrentAssetInvestmentAllocation, Instruction, ActionType, SwapDetailsExactIn, OptionalSwapDetailsExactIn, OptionalSwapDetailsExactOut, DexDistribution };
use soroban_sdk::{Address, Env, Map, Vec, String};
use crate::storage::{get_config};


pub fn calculate_rebalance(
    e: &Env,
    current_allocations: Map<Address, CurrentAssetInvestmentAllocation>,
    prices: Vec<AssetPrice>,
    ratios: Vec<AssetRatio>
) -> Vec<Instruction> {
// ) -> Map<Address, i128> {
    // Create a vector to store rebalancing instructions
    // let instructions = Vec::new(e);

    // For now, return empty instructions
    // TODO: Implement rebalancing logic based on:
    // 1. Compare current allocations with desired ratios from Config
    // 2. Generate Swap/Invest/Withdraw instructions to achieve target allocation
    // 3. Consider transaction costs and minimum trade sizes

    // Get total weight
    let total_weight: i128 = ratios.iter().map(|r| r.ratio).sum();

    // get total portfolio
    let total_portfolio_value = calculate_total_portfolio_value(&e, &current_allocations, &prices);

    // get deviations
    let deviations = calculate_deviations(
        &e,
        &current_allocations,
        &prices,
        &ratios,
        total_portfolio_value,
        total_weight,
    );

    let instructions = create_swap_instructions(&e, &deviations, &prices);
    // Build instructions 
    // deviations
    instructions
}

fn calculate_total_portfolio_value(
    env: &Env,
    current_allocations: &Map<Address, CurrentAssetInvestmentAllocation>,
    prices: &Vec<AssetPrice>,
) -> i128 {
    let mut total_portfolio_value: i128 = 0;

    // Iterate over current allocations
    for (_, allocation) in current_allocations.iter() {
        // Find the corresponding price for the asset
        if let Some(price_entry) = prices.iter().find(|p| p.asset == allocation.asset) {
            // Multiply total_amount by price and add to total_portfolio_value
            total_portfolio_value += allocation.total_amount * price_entry.price;
        }
    }

    total_portfolio_value
}


fn calculate_deviations(
    env: &Env,
    current_allocations: &Map<Address, CurrentAssetInvestmentAllocation>,
    prices: &Vec<AssetPrice>,
    ratios: &Vec<AssetRatio>,
    total_portfolio_value: i128,
    total_weight: i128,
) -> Map<Address, i128> {
    let mut deviations: Map<Address, i128> = Map::new(env);

    // Iterate over current allocations
    for (address, allocation) in current_allocations.iter() {
        // Find corresponding price for the asset
        if let Some(price_entry) = prices.iter().find(|p| p.asset == allocation.asset) {
            // Find corresponding weight ratio for the asset
            if let Some(ratio_entry) = ratios.iter().find(|r| r.asset == allocation.asset) {
                // Compute target allocation
                let target_allocation = (ratio_entry.ratio as i128 * total_portfolio_value)
                    / total_weight as i128;

                // Compute actual allocation
                let actual_allocation = allocation.total_amount * price_entry.price;

                // Compute deviation
                let deviation = actual_allocation - target_allocation;

                // Store deviation indexed by address
                deviations.set(address, deviation/price_entry.price);
            }
        }
    }

    deviations
}

fn create_swap_instructions(
    env: &Env,
    deviations: &Map<Address, i128>,
    prices: &Vec<AssetPrice>,
) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new(env);

    // Extract the tokens with positive and negative deviations
    let mut token_in: Option<Address> = None;
    let mut token_out: Option<Address> = None;
    let mut amount_in: i128 = 0;

    for (address, deviation) in deviations.iter() {
        if deviation > 0 {
            token_in = Some(address.clone());
            amount_in = deviation; // Use the absolute value
        } else if deviation < 0 {
            token_out = Some(address.clone());
        }
    }

    // Ensure both tokens are present
    if let (Some(token_in), Some(token_out)) = (token_in, token_out) {
        // Create distribution
        let distribution = Vec::from_array(
            env,
            [DexDistribution {
                protocol_id: String::from_str(&env,"soroswap"),
                path: Vec::from_array(env, [token_in.clone(), token_out.clone()]),
                parts: 1,
            }],
        );

        // Create swap details
        let swap_details = SwapDetailsExactIn {
            token_in: token_in.clone(),
            token_out: token_out.clone(),
            amount_in,
            amount_out_min: 0, // Placeholder, can be estimated based on price slippage
            distribution,
            deadline: env.ledger().timestamp() + 3600u64, // 10 minutes from now
        };

        // Create instruction
        let instruction = Instruction {
            action: ActionType::SwapExactIn,
            strategy: None,
            amount: None,
            swap_details_exact_in: OptionalSwapDetailsExactIn::Some(swap_details),
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        };

        // Add to the instructions list
        instructions.push_back(instruction);
    }

    instructions
}
