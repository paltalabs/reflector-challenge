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
    BytesN,
    Symbol,
};
use soroswap_setup::{create_soroswap_factory, create_soroswap_pool, create_soroswap_router, SoroswapFactoryClient, SoroswapRouterClient};
use std::vec;

use crate::{TrustlessManager, TrustlessManagerClient, AssetRatio};

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
use defindex_vault::DeFindexVaultClient;
use hodl_strategy::HodlStrategyClient;
use reflector::ReflectorClient;

// USE MODELS
pub use reflector::{ConfigData, Asset, PriceData};
pub use defindex_factory::{AssetStrategySet, Strategy};
pub use defindex_vault::{AssetInvestmentAllocation, StrategyAllocation};

// // The configuration parameters for the contract.
// pub struct ConfigData {
//     // The admin address.
//     pub admin: Address,
//     // The retention period for the prices.
//     pub period: u64,
//     // The assets supported by the contract.
//     pub assets: Vec<Asset>,
//     // The base asset for the prices.
//     pub base_asset: Asset,
//     // The number of decimals for the prices.
//     pub decimals: u32,
//     // The resolution of the prices.
//     pub resolution: u32,
// }



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

    let args = (
        admin.clone(), 
        defindex_receiver.clone(), 
        defindex_fee.clone(), 
        defindex_wasm_hash.clone());
    let address = &e.register(defindex_factory::WASM, args);
    DeFindexFactoryClient::new(e, address)
}
fn create_hodl_strategy<'a>(
    e: &Env, 
    asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val> = sorobanvec![&e];
    let args = (asset, init_args);

    let address = &e.register(hodl_strategy::WASM, args);
    let hodl_strategy = HodlStrategyClient::new(e, address);
    hodl_strategy
}
// fn create_defindex_vault<'a>(
//     e: &Env, 
//     admin: &Address, 
//     defindex_receiver: &Address, 
//     defindex_factory: &Address) -> DeFindexVaultClient<'a> {
//     let args = (admin.clone(), defindex_receiver.clone(), defindex_factory.clone());
//     let address = &e.register(defindex_vault::WASM, args);
//     DeFindexVaultClient::new(e, address)
// }
fn create_reflector<'a>(
    e: &Env,
    config_data: &ConfigData
) -> ReflectorClient<'a> {
    let address = &e.register(reflector::WASM, ());
    let reflector_client = ReflectorClient::new(e, address);
    reflector_client.config(config_data);
    reflector_client
}

// THE CONTRACT TO BE TESTED
fn create_trustless_manager<'a>(
    e: &Env,
    vault: &Address,
    oracle: &Address,
    asset_ratios: &Vec<AssetRatio>
) -> TrustlessManagerClient<'a> {
    // __constructor(e: Env, vault: Address, oracle: Address, asset_ratios: Vec<AssetRatio>
    let args = (vault.clone(), oracle.clone(), asset_ratios.clone());
    let address = &e.register(TrustlessManager, args);
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
    defindex_vault: DeFindexVaultClient<'a>,
    admin: Address,
    defindex_receiver: Address,
    token_0_admin_client: SorobanTokenAdminClient<'a>,
    token_0: SorobanTokenClient<'a>,
    token_0_admin: Address,
    token_1_admin_client: SorobanTokenAdminClient<'a>,
    token_1: SorobanTokenClient<'a>,
    token_1_admin: Address,
    strategy_client_token_0: HodlStrategyClient<'a>,
    strategy_client_token_1: HodlStrategyClient<'a>,
    reflector: ReflectorClient<'a>,
    trustless_manager: TrustlessManagerClient<'a>,
    soroswap_router: SoroswapRouterClient<'a>,
    soroswap_factory: SoroswapFactoryClient<'a>,
    user: Address,
}

impl<'a> TrustlessManagerTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.budget().reset_unlimited();
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
            
        // HODL STRATEGIES
        let strategy_client_token_0 = create_hodl_strategy(&env, &token_0.address);
        let strategy_client_token_1 = create_hodl_strategy(&env, &token_1.address);


        // DEFINDEX VAULT WITH ADMIN AS MANAGER
        let token_0_hodl_strategy_set = AssetStrategySet {
            address: token_0.address.clone(),
            strategies: sorobanvec![
                &env,
                Strategy {
                    address: strategy_client_token_0.address.clone(),
                    name: String::from_str(&env, "TOKEN 0 HODL"),
                    paused: false,
                }
            ],
        };
       
        let token_1_hodl_strategy_set = AssetStrategySet {
            address: token_1.address.clone(),
            strategies: sorobanvec![
                &env,
                Strategy {
                    address: strategy_client_token_1.address.clone(),
                    name: String::from_str(&env, "TOKEN 1 HODL"),
                    paused: false,
                }
            ],
        };
        let asset_params = sorobanvec![
            &env,
            token_0_hodl_strategy_set,
            token_1_hodl_strategy_set
        ];
        let salt = BytesN::from_array(&env, &[0; 32]);
        defindex_factory.create_defindex_vault( 
            &admin, // emergency_manager
            &admin, // fee_receiver
            &2000u32, // vault_fee
            &String::from_str(&env, "XLM-XRP 50/50 HODL"), // name
            &String::from_str(&env, "XLM-XRP-50-50-HODL"), // symbol
            &admin, //manager
            &asset_params,
            &salt
          );

        let defindex_vault_address = defindex_factory.deployed_defindexes().get(0).unwrap();
        let defindex_vault = DeFindexVaultClient::new(&env, &defindex_vault_address);
        
        // REFLECTOR ORACLE
        // https://stellar.expert/explorer/public/contract/CAFJZQWSED6YAWZU3GWRTOCNPPCGBN32L7QV43XX5LZLFTK6JLN34DLN/storage?durability=instance
        // assets should be something like this
        // assets: [["Stellar"sym, token_0.address], ["Stellar"sym, token_1.address]

        let config_data = ConfigData {
            admin: admin.clone(),
            period: 86400000,
            assets: sorobanvec![
                &env,    
                Asset::Other(Symbol::new(&env, "XLM")),
                Asset::Other(Symbol::new(&env, "XRP")),
            ],
            base_asset: Asset::Stellar(token_0.address.clone()),// This is supposed to be USDC.
            // in our case not needed to have it
            decimals: 8,
            resolution: 8,
        };
        let reflector = create_reflector(&env, &config_data);
        
        // TRUSTLESS MANAGER  
        let trustless_manager = create_trustless_manager(
            &env,
            &defindex_vault_address,
            &reflector.address,
            &sorobanvec![
                &env,
                AssetRatio {
                    asset: token_0.address.clone(),
                    symbol: Symbol::new(&env, "XLM"),
                    ratio: 1000,
                },
                AssetRatio {
                    asset: token_1.address.clone(),
                    symbol: Symbol::new(&env, "XRP"),
                    ratio: 1000,
                },
            ]
        );

        // ADMIN DEPOSITS 1000 XLM, => 500 USD & 200 XRP, => 500 USD into the vault
        /*
        
        
- manager (admin) executes deposit function in vault (check invest test in defindex)
// Prepare investments object
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token0.address.clone(),
            amount: 100,
            }),
        ],
    }),
    Some(AssetInvestmentAllocation {
        asset: test.token1.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token1.address.clone(),
            amount: 200,
            }),
        ],
    })];

    defindex_contract.invest(
        &asset_investments,
    );

        
        
        */
        // admin executes investment in vault

        // VAULT ADMIN PASSES TRUSTLESS_MANAGER TO VAULT
        defindex_vault.set_manager(&trustless_manager.address);
                        
        // Soroswap Pools
        let soroswap_admin = Address::generate(&env);
        let soroswap_factory = create_soroswap_factory(&env, &soroswap_admin);

        token_0_admin_client.mint(&soroswap_admin, &100_000_000_0000000);
        token_1_admin_client.mint(&soroswap_admin, &5_000_000_0000000);

        let soroswap_router = create_soroswap_router(&env, &soroswap_factory.address);
        create_soroswap_pool(&env, &soroswap_router, &soroswap_admin, &token_0.address, &token_1.address, &100_000_000_0000000, &5_000_000_0000000);

        let user = Address::generate(&env);
        env.budget().reset_unlimited();

        TrustlessManagerTest {
            env,
            defindex_factory,
            admin,
            defindex_receiver,
            defindex_vault,
            token_0_admin_client,
            token_0,
            token_0_admin,
            token_1_admin_client,
            token_1,
            token_1_admin,
            strategy_client_token_0,
            strategy_client_token_1,
            reflector,
            trustless_manager,
            soroswap_router,
            soroswap_factory,
            user,
        }
    }
}

// mod vault;
mod utils;
mod setup;
mod oracle;
mod soroswap_setup;
mod trustless_manager;
