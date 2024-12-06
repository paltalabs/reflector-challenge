use soroban_sdk::{contracttype, Address, Env, Vec};

use crate::model::AssetRatio;

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    AssetRatios,
    Vault
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
pub fn set_asset_ratios(e: &Env, asset_ratios: Vec<AssetRatio>) {
    e.storage().instance().set(&DataKey::AssetRatios, &asset_ratios);
}

pub fn get_asset_ratios(e: &Env) -> Vec<AssetRatio> {
    e.storage().instance().get(&DataKey::AssetRatios).unwrap()
}

pub fn set_vault(e: &Env, vault: Address) {
    e.storage().instance().set(&DataKey::Vault, &vault);
}

pub fn get_vault(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Vault).unwrap()
}
