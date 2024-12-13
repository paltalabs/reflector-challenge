#![cfg(test)]
extern crate std;
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{
    testutils::Address as _, 
    vec as sorobanvec, 
    Address, 
    Env, 
    String, 
    Val, 
    Vec, 
    BytesN};
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
use defindex_factory::DeFindexFactoryClient;
use defindex_vault::{DeFindexVaultClient, Strategy};
use hodl_strategy::HodlStrategyClient;
use reflector::ReflectorClient;


// fn create_strategy_contract<'a>(e: &Env, asset: &Address, init_args: &Vec<Val>) -> StrategyContractClient<'a> {
//     let args = (asset.clone(), init_args.clone());

//     let address = &e.register(hodl_strategy::WASM, args);
//     let strategy = StrategyContractClient::new(e, address); 
//     strategy
// }  

// Deploy Contracts
fn create_defindex_factory<'a>(
    e: &Env, 
    admin: &Address, 
    defindex_receiver: &Address, 
    defindex_fee: &u32,
    defindex_wasm_hash: &BytesN<32>) -> DeFindexFactoryClient<'a> {
    
    // fn __constructor(
    //     e: Env, 
    //     admin: Address,
    //     defindex_receiver: Address,
    //     defindex_fee: u32,
    //     vault_wasm_hash: BytesN<32>
    // );
    
    let args = (
        admin.clone(), 
        defindex_receiver.clone(), 
        defindex_fee.clone(), 
        defindex_wasm_hash.clone());
    let address = &e.register(defindex_factory::WASM, args);
    DeFindexFactoryClient::new(e, address)
}
fn create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val> = sorobanvec![&e];
    let args = (asset, init_args);

    let address = &e.register(hodl_strategy::WASM, args);
    let hodl_strategy = HodlStrategyClient::new(e, address);
    hodl_strategy
}
fn create_reflector<'a>(e: &Env) -> ReflectorClient<'a> {
    let address = &e.register(reflector::WASM, ());
    ReflectorClient::new(e, address)
}

// THE CONTRACT TO BE TESTED
fn create_trustless_manager<'a>(e: &Env) -> TrustlessManagerClient<'a> {
    let address = &e.register(TrustlessManager, ());
    TrustlessManagerClient::new(e, address)
    
}

// TOKEN RELATED FUNCTIONS
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> SorobanTokenClient<'a> {
    SorobanTokenClient::new(
        e,
        &e.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}
pub(crate) fn get_token_admin_client<'a>(
    e: &Env,
    address: &Address,
) -> SorobanTokenAdminClient<'a> {
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

pub(crate) fn generate_random_users(e: &Env, users_count: u32) -> vec::Vec<Address> {
        let mut users = vec![];
        for _c in 0..users_count {
            users.push(Address::generate(e));
        }
        users
}

pub struct TrustlessManagerTest<'a> {
    env: Env,
    defindex_factory: DeFindexFactoryClient<'a>,
    // defindex_vault: DeFindexVaultClient<'a>,
    admin: Address,
    defindex_receiver: Address,
    token_0_admin_client: SorobanTokenAdminClient<'a>,
    token_0: SorobanTokenClient<'a>,
    token_0_admin: Address,
    token_1_admin_client: SorobanTokenAdminClient<'a>,
    token_1: SorobanTokenClient<'a>,
    token_1_admin: Address,
    emergency_manager: Address,
    vault_fee_receiver: Address,
    defindex_protocol_receiver: Address,
    // strategy_client_token_0: HodlStrategyClient<'a>,
    // strategy_client_token_1: HodlStrategyClient<'a>,
    // reflector: ReflectorClient<'a>,
    // trustless_manager: TrustlessManagerClient<'a>,
    user: Address,
}

impl<'a> TrustlessManagerTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        // DEFINDEX FACTORY
        let admin = Address::generate(&env);
        let defindex_receiver = Address::generate(&env);
        let defindex_vault_wasm_hash = env.deployer().upload_contract_wasm(defindex_vault::WASM);
        let defindex_factory = create_defindex_factory(
            &env, 
            &admin, 
            &defindex_receiver, 
            &100u32, 
            &defindex_vault_wasm_hash);
            
        // TEST TOKENS
        let token_0_admin = Address::generate(&env);
        let token_1_admin = Address::generate(&env);
        
        let token_0 = create_token_contract(&env, &token_0_admin);
        let token_1 = create_token_contract(&env, &token_1_admin);
        
        let token_0_admin_client = get_token_admin_client(&env, &token_0.address.clone());
        let token_1_admin_client = get_token_admin_client(&env, &token_1.address.clone());
            
            // let strategy_client_token_0 = create_hodl_strategy(&env, &token_0.address);
            // let strategy_client_token_1 = create_hodl_strategy(&env, &token_1.address);
            
            // let reflector = create_reflector(&env);
            // let trustless_manager = create_trustless_manager(&env);
            
            // DEFINDEX PROTOCOL
            
            let emergency_manager = Address::generate(&env);
            let vault_fee_receiver = Address::generate(&env);
            let defindex_protocol_receiver = Address::generate(&env);
            let manager = Address::generate(&env);

            let user = Address::generate(&env);
            env.budget().reset_unlimited();
        TrustlessManagerTest {
            env,
            defindex_factory,
            admin,
            defindex_receiver,
            // defindex_vault,
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
            // strategy_client_token_0,
            // strategy_client_token_1,
            // reflector,
            // trustless_manager,
            user,
        }
    }
}

// mod vault;
mod utils;
mod setup;
