use crate::model::AssetRatio;
use soroban_sdk::{Address, Env, Map, String, Vec};

pub trait ManagerTrait {
    fn hello_world(e: Env) -> String;
    fn rebalance(e: Env);
}
