use soroban_sdk::{Address, Env};

use super::DataKey;

pub fn read_max_components(e: &Env) -> Option<u32> {
    let key = DataKey::MaxComponents;
    e.storage().instance().get(&key)
}

pub fn write_max_components(e: &Env, val: u32) {
    let key = DataKey::MaxComponents;
    e.storage().instance().set(&key, &val);
}
