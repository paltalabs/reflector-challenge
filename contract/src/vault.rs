use crate::model::{CurrentAssetInvestmentAllocation, StrategyAllocation};
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

pub trait VaultTrait {
    fn fetch_current_idle_funds(e: Env) -> Map<Address, CurrentAssetInvestmentAllocation>;
}

#[contract]
pub struct Vault;

#[contractimpl]
impl VaultTrait for Vault {
    fn fetch_current_idle_funds(e: Env) -> Map<Address, CurrentAssetInvestmentAllocation> {
        // Create a new map to store the results
        let mut funds_map: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(&e);

        // For demonstration, let's create some hardcoded sample data
        // In a real implementation, this would fetch actual data from storage

        // Create a sample token address
        let token_XRP = Address::from_str(&e, "CACY3RX5UGOG43AZ5O4SVWRPPVXLTHBU3CKPFLCRPB5BY46LES6JLYOR");
        let token_XLM = Address::from_str(&e, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
        let hodl_xlm= Address::from_str(&e, "CCJO5RFBQDGN3VMY6AUEGWU2G7LEKEN7TCHOJYGLA26QN55BOUAWCJV4");
        let hodl_xrp = Address::from_str(&e, "CAHWETYSTW2WEW3RRSOZI6QVBY5MJX75EA3ZBGLSP5HXU2JDE23S2LTL");

        // Create sample strategy allocations
        let mut strategy_allocations = Vec::new(&e);
        strategy_allocations.push_back(StrategyAllocation {
            strategy_address: hodl_xlm
            amount: 1000,
        });
        strategy_allocations.push_back(StrategyAllocation {
            strategy_address: hodl_xrp,
            amount: 2000,
        });

        // Create sample allocation data
        let allocation = CurrentAssetInvestmentAllocation {
            asset: token_XRP.clone(),
            total_amount: 5000,
            idle_amount: 2000,
            invested_amount: 3000,
            strategy_allocations,
        };

        // Add the sample data to the map
        funds_map.set(token_a, allocation);

        funds_map
    }
}
