use soroban_sdk::{contracttype, Env};

use crate::model::Config;

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Config
}

const DAY_IN_LEDGERS: u32 = 17280;
pub const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

// AssetRatio
pub fn set_config(e: &Env, config: Config) {
    e.storage().instance().set(&DataKey::Config, &config);
}

pub fn get_config(e: &Env) -> Config {
    e.storage().instance().get(&DataKey::Config).unwrap()
}