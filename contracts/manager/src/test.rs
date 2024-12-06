#![cfg(test)]
extern crate std;
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{testutils::Address as _, vec as sorobanvec, Address, Env, String, Vec, Val};
use std::vec;

// IMPORT WASMS
pub mod defindex_vault {
    soroban_sdk::contractimport!(file = "../defindex_vault.wasm");
    pub type DeFindexVaultClient<'a> = Client<'a>;
}
pub mod defindex_factory {
    soroban_sdk::contractimport!(file = "../defindex_factory.wasm");
    pub type DeFindexFactoryClient<'a> = Client<'a>;
}
pub mod hodl_strategy {
    soroban_sdk::contractimport!(file = "../hodl_strategy.wasm");
    pub type HodlStrategyClient<'a> = Client<'a>;
}
pub mod reflector {
    soroban_sdk::contractimport!(file = "../reflector.wasm");
    pub type ReflectorClient<'a> = Client<'a>;
}


// USE CLIENTS
use hodl_strategy::HodlStrategyClient;
use defindex_factory::DeFindexFactoryClient;
use defindex_vault::{DeFindexVaultClient, Strategy};
use reflector::ReflectorClient;

// // Deploy Contracts
// fn create_defindex_vault<'a>(e: &Env) -> DeFindexVaultClient<'a> {
//     let address = &e.register_contract_wasm(None, defindex_vault::WASM);
//     DeFindexVaultClient::new(e, address)
// }
// fn create_defindex_factory<'a>(e: & Env) -> DeFindexFactoryClient<'a> {
//     let address = &e.register_contract_wasm(None, defindex_factory::WASM);
//     DeFindexFactoryClient::new(e, address)
// }
fn create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val>= sorobanvec![&e];
    let args = (asset, init_args);

    let address = &e.register(hodl_strategy::WASM, args);
    let hodl_strategy = HodlStrategyClient::new(e, address);
    hodl_strategy
}
// pub fn create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
//     let init_args: Vec<Val>= vec![e];

//     let args = (asset, init_args);
//     HodlStrategyClient::new(e, &e.register(HodlStrategy, args))
// }
// pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> SorobanTokenClient<'a> {
//     SorobanTokenClient::new(e,&e.register_stellar_asset_contract_v2(admin.clone()).address())
// }

// pub(crate) fn get_token_admin_client<'a>(
//     e: &Env,
//     address: &Address,
// ) -> SorobanTokenAdminClient<'a> {
//     SorobanTokenAdminClient::new(e, address)
// }

// pub(crate) fn create_strategy_params_token0(test: &DeFindexVaultTest) -> Vec<Strategy> {
//     sorobanvec![
//         &test.env, 
//         Strategy {
//             name: String::from_str(&test.env, "Strategy 1"),
//             address: test.strategy_client_token0.address.clone(),
//             paused: false,
//         }
//     ]
// }

// pub(crate) fn create_strategy_params_token1(test: &DeFindexVaultTest) -> Vec<Strategy> {
//     sorobanvec![
//         &test.env,
//         Strategy {
//             name: String::from_str(&test.env, "Strategy 1"),
//             address: test.strategy_client_token1.address.clone(),
//             paused: false,
//         }
//     ]
// }

// pub struct DeFindexVaultTest<'a> {
//     env: Env,
//     defindex_factory: Address,
//     defindex_contract: DeFindexVaultClient<'a>,
//     token0_admin_client: SorobanTokenAdminClient<'a>,
//     token0: SorobanTokenClient<'a>,
//     token0_admin: Address,
//     token1_admin_client: SorobanTokenAdminClient<'a>,
//     token1: SorobanTokenClient<'a>,
//     token1_admin: Address,
//     emergency_manager: Address,
//     vault_fee_receiver: Address,
//     defindex_protocol_receiver: Address,
//     manager: Address,
//     strategy_client_token0: HodlStrategyClient<'a>,
//     strategy_client_token1: HodlStrategyClient<'a>,
// }

// impl<'a> DeFindexVaultTest<'a> {
//     fn setup() -> Self {
//         let env = Env::default();
//         // env.mock_all_auths();

//         // Mockup, should be the factory contract
//         let defindex_factory = Address::generate(&env);

//         let defindex_contract = create_defindex_vault(&env);

//         let emergency_manager = Address::generate(&env);
//         let vault_fee_receiver = Address::generate(&env);
//         let defindex_protocol_receiver = Address::generate(&env);
//         let manager = Address::generate(&env);

//         let token0_admin = Address::generate(&env);
//         let token0 = create_token_contract(&env, &token0_admin);

//         let token1_admin = Address::generate(&env);
//         let token1 = create_token_contract(&env, &token1_admin);

//         let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());
//         let token1_admin_client = get_token_admin_client(&env, &token1.address.clone());

//         // token1_admin_client.mint(to, amount);

//         let strategy_client_token0 = create_hodl_strategy(&env, &token0.address);
//         let strategy_client_token1 = create_hodl_strategy(&env, &token1.address);

//         env.budget().reset_unlimited();
        
//         DeFindexVaultTest {
//             env,
//             defindex_factory,
//             defindex_contract,
//             token0_admin_client,
//             token0,
//             token0_admin,
//             token1_admin_client,
//             token1,
//             token1_admin,
//             emergency_manager,
//             vault_fee_receiver,
//             defindex_protocol_receiver,
//             manager,
//             strategy_client_token0,
//             strategy_client_token1,
//         }
//     }

//     pub(crate) fn generate_random_users(e: &Env, users_count: u32) -> vec::Vec<Address> {
//         let mut users = vec![];
//         for _c in 0..users_count {
//             users.push(Address::generate(e));
//         }
//         users
//     }
// }

// mod vault;
