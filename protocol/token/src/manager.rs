use soroban_sdk::{Address, Env};

use crate::types::DataKey;

pub fn has_manager(e: &Env) -> bool {
    let key = DataKey::Manager;
    e.storage().instance().has(&key)
}

pub fn read_manager(e: &Env) -> Address {
    let key = DataKey::Manager;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_manager(e: &Env, id: &Address) {
    let key = DataKey::Manager;
    e.storage().instance().set(&key, id);
}
