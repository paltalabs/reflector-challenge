use soroban_sdk::{contract, contractimpl, Address, Env, Map, Vec};

soroban_sdk::contractimport!(file = "../vault.wasm");

pub trait VaultTrait {
    fn fetch_total_managed_funds(e: Env) -> Map<Address, CurrentAssetInvestmentAllocation>;
}

#[contract]
pub struct Vault;

#[contractimpl]
impl VaultTrait for Vault {
    fn fetch_total_managed_funds(e: Env) -> Map<Address, CurrentAssetInvestmentAllocation> {
        // Create a new map to store the results
        let mut funds_map: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(&e);

        // Create sample token addresses
        let token_XRP = Address::from_str(
            &e,
            "CACY3RX5UGOG43AZ5O4SVWRPPVXLTHBU3CKPFLCRPB5BY46LES6JLYOR",
        );
        let token_XLM = Address::from_str(
            &e,
            "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
        );
        let hodl_xlm = Address::from_str(
            &e,
            "CCJO5RFBQDGN3VMY6AUEGWU2G7LEKEN7TCHOJYGLA26QN55BOUAWCJV4",
        );
        let hodl_xrp = Address::from_str(
            &e,
            "CAHWETYSTW2WEW3RRSOZI6QVBY5MJX75EA3ZBGLSP5HXU2JDE23S2LTL",
        );

        // Create XRP allocation
        let mut xrp_strategy_allocations = Vec::new(&e);
        xrp_strategy_allocations.push_back(StrategyAllocation {
            strategy_address: hodl_xrp,
            amount: 2000,
        });

        let xrp_allocation = CurrentAssetInvestmentAllocation {
            asset: token_XRP.clone(),
            total_amount: 5000,
            idle_amount: 3000,
            invested_amount: 2000,
            strategy_allocations: xrp_strategy_allocations,
        };

        // Create XLM allocation
        let mut xlm_strategy_allocations = Vec::new(&e);
        xlm_strategy_allocations.push_back(StrategyAllocation {
            strategy_address: hodl_xlm,
            amount: 1500,
        });

        let xlm_allocation = CurrentAssetInvestmentAllocation {
            asset: token_XLM.clone(),
            total_amount: 4000,
            idle_amount: 2500,
            invested_amount: 1500,
            strategy_allocations: xlm_strategy_allocations,
        };

        // Add both allocations to the map
        funds_map.set(token_XRP, xrp_allocation);
        funds_map.set(token_XLM, xlm_allocation);

        funds_map
    }
}
