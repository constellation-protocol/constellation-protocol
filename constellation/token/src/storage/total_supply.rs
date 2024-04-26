use super::keys::DataKey;
use super::types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use crate::error::Error;
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_total_supply(e: &Env) -> i128 {
    let key = DataKey::TotalSupply;
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        _extend_ttl(e);
        balance
    } else {
        0
    }
}

pub fn write_total_supply(e: &Env, amount: i128) {
    let key = DataKey::TotalSupply;
    e.storage().persistent().set(&key, &amount);
    _extend_ttl(e);
}

fn _extend_ttl(e: &Env) {
    let key = DataKey::TotalSupply;
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}
