use crate::model::{AssetPrice, AssetRatio};
use crate::vault::{CurrentAssetInvestmentAllocation, Instruction, ActionType, SwapDetailsExactIn, OptionalSwapDetailsExactIn, OptionalSwapDetailsExactOut, DexDistribution };
use soroban_sdk::{Address, Env, Map, Vec, String};

pub fn calculate_rebalance(
    e: &Env,
    current_allocations: Map<Address, CurrentAssetInvestmentAllocation>,
    prices: Vec<AssetPrice>,
    ratios: Vec<AssetRatio>,
    router: Address,
    pair: Address,
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
    let total_portfolio_value = calculate_total_portfolio_value(&current_allocations, &prices);

    // get deviations
    let deviations = calculate_deviations(
        &e,
        &current_allocations,
        &prices,
        &ratios,
        total_portfolio_value,
        total_weight,
    );
    let (withdraw_instructions, invest_instructions ) = create_invest_withdraw_instructions(&e, &deviations, &current_allocations);

    let swap_instructions = create_swap_instructions(&e, &deviations, &prices, &router, &pair);
    // Build instructions 
    // deviations
    let ordered_instructions = order_instructions(
        &e,
        withdraw_instructions,
        swap_instructions,
        invest_instructions,
    );
    ordered_instructions
}

fn calculate_total_portfolio_value(
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
    _prices: &Vec<AssetPrice>,
    router: &Address,
    pair: &Address,
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
            router: router.clone(),
            pair: pair.clone(),
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

fn create_invest_withdraw_instructions(
    env: &Env,
    deviations: &Map<Address, i128>,
    current_allocations: &Map<Address, CurrentAssetInvestmentAllocation>,
) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut withdraw_instructions: Vec<Instruction> = Vec::new(env);
    let mut invest_instructions: Vec<Instruction> = Vec::new(env);

    // if the swapped amount is not enough to be invested, The invest instructions may fail
    let mut factor_with_slippage_bps: i128;

    for (address, deviation) in deviations.iter() {
        // Get the current allocation for the asset
        if let Some(allocation) = current_allocations.get(address.clone()) {
            let action = if deviation > 0 {
                ActionType::Withdraw
            } else {
                ActionType::Invest
            };

            if action == ActionType::Withdraw {
                factor_with_slippage_bps = 10000
            } else {
                factor_with_slippage_bps = 9000
            }

            let amount = deviation.abs()*factor_with_slippage_bps/10000;

            // Retrieve the strategy address (we'll use the first one for simplicity)
            let strategy_address = allocation.strategy_allocations.get(0).map(|sa| sa.strategy_address);

            // Create the instruction
            let instruction = Instruction {
                action,
                strategy: strategy_address,
                amount: Some(amount),
                swap_details_exact_in: OptionalSwapDetailsExactIn::None,
                swap_details_exact_out: OptionalSwapDetailsExactOut::None,
            };

            // Add to the appropriate list of instructions
            if action == ActionType::Withdraw {
                withdraw_instructions.push_back(instruction);
            } else {
                invest_instructions.push_back(instruction);
            }
        }
    }

    (withdraw_instructions, invest_instructions)
}


fn order_instructions(
    env: &Env,
    withdraw_instructions: Vec<Instruction>,
    swap_instructions: Vec<Instruction>,
    invest_instructions: Vec<Instruction>,
) -> Vec<Instruction> {
    let mut ordered_instructions: Vec<Instruction> = Vec::new(env);

    // Add withdraw instructions first
    for instruction in withdraw_instructions.iter() {
        ordered_instructions.push_back(instruction.clone());
    }

    // Add swap instructions second
    for instruction in swap_instructions.iter() {
        ordered_instructions.push_back(instruction.clone());
    }

    // Add invest instructions last
    for instruction in invest_instructions.iter() {
        ordered_instructions.push_back(instruction.clone());
    }

    ordered_instructions
}