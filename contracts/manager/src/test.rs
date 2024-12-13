#![cfg(test)]
extern crate std;
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{testutils::Address as _, vec as sorobanvec, Address, Env, String, Vec, Val};
use std::vec;

use crate::{TrustlessManager, TrustlessManagerClient};

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

// Deploy Contracts
fn create_defindex_vault<'a>(e: &Env) -> DeFindexVaultClient<'a> {
    // let address = &e.register_contract_wasm(None, defindex_vault::WASM);
    let address = &e.register(defindex_vault::WASM, ());
    DeFindexVaultClient::new(e, address)
}
fn create_defindex_factory<'a>(e: & Env) -> DeFindexFactoryClient<'a> {
    // let address = &e.register_contract_wasm(None, defindex_factory::WASM);
    let address = &e.register(defindex_factory::WASM, ());
    DeFindexFactoryClient::new(e, address)
}
fn create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val>= sorobanvec![&e];
    let args = (asset, init_args);

    let address = &e.register(hodl_strategy::WASM, args);
    let hodl_strategy = HodlStrategyClient::new(e, address);
    hodl_strategy
}
fn create_trustless_manager<'a>(e: &Env) -> TrustlessManagerClient<'a> {
    let address = &e.register(TrustlessManager, ());
    HodlStrategyClient::new(e, address)
    
}


// TOKEN RELATED FUNCTIONS
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> SorobanTokenClient<'a> {
    SorobanTokenClient::new(e,&e.register_stellar_asset_contract_v2(admin.clone()).address())
}
pub(crate) fn get_token_admin_client<'a>(e: &Env,address: &Address) -> SorobanTokenAdminClient<'a> {
    SorobanTokenAdminClient::new(e, address)
}

// pub(crate) fn create_strategy_params_token_0(test: &TrustlessManagerTest) -> Vec<Strategy> {
//     sorobanvec![
//         &test.env, 
//         Strategy {
//             name: String::from_str(&test.env, "Strategy 1"),
//             address: test.strategy_client_token_0.address.clone(),
//             paused: false,
//         }
//     ]
// }

// pub(crate) fn create_strategy_params_token_1(test: &TrustlessManagerTest) -> Vec<Strategy> {
//     sorobanvec![
//         &test.env,
//         Strategy {
//             name: String::from_str(&test.env, "Strategy 1"),
//             address: test.strategy_client_token_1.address.clone(),
//             paused: false,
//         }
//     ]
// }

pub struct TrustlessManagerTest<'a> {
    env: Env,
    defindex_factory: Address,
    defindex_vault: DeFindexVaultClient<'a>,
    token_0_admin_client: SorobanTokenAdminClient<'a>,
    token_0: SorobanTokenClient<'a>,
    token_0_admin: Address,
    token_1_admin_client: SorobanTokenAdminClient<'a>,
    token_1: SorobanTokenClient<'a>,
    token_1_admin: Address,
    emergency_manager: Address,
    vault_fee_receiver: Address,
    defindex_protocol_receiver: Address,
    // manager: Address,
    strategy_client_token_0: HodlStrategyClient<'a>,
    strategy_client_token_1: HodlStrategyClient<'a>,
}

impl<'a> TrustlessManagerTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let defindex_factory = Address::generate(&env);
        let defindex_vault = create_defindex_vault(&env);

        let emergency_manager = Address::generate(&env);
        let vault_fee_receiver = Address::generate(&env);
        let defindex_protocol_receiver = Address::generate(&env);
        let manager = Address::generate(&env);

        let token_0_admin = Address::generate(&env);
        let token_1_admin = Address::generate(&env);
        
        let token_0 = create_token_contract(&env, &token_0_admin);
        let token_1 = create_token_contract(&env, &token_1_admin);

        let token_0_admin_client = get_token_admin_client(&env, &token_0.address.clone());
        let token_1_admin_client = get_token_admin_client(&env, &token_1.address.clone());


        let strategy_client_token_0 = create_hodl_strategy(&env, &token_0.address);
        let strategy_client_token_1 = create_hodl_strategy(&env, &token_1.address);

        env.budget().reset_unlimited();
        
        TrustlessManagerTest {
            env,
            defindex_factory,
            defindex_vault,
            token_0_admin_client,
            token_0,
            token_0_admin,
            token_1_admin_client,
            token_1,
            token_1_admin,
            emergency_manager,
            vault_fee_receiver,
            defindex_protocol_receiver,
//             manager,
            strategy_client_token_0,
            strategy_client_token_1,
        }
    }

//     pub(crate) fn generate_random_users(e: &Env, users_count: u32) -> vec::Vec<Address> {
//         let mut users = vec![];
//         for _c in 0..users_count {
//             users.push(Address::generate(e));
//         }
//         users
//     }
}

// mod vault;
