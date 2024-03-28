use soroban_sdk::{Address, Env};

use super::keys::DataKey;

pub fn has_registry(e: &Env) -> bool {
    let key = DataKey::Registry;
    e.storage().instance().has(&key)
}

pub fn read_registry(e: &Env) -> Option<Address> {
    let key = DataKey::Registry;
    e.storage().instance().get(&key)
}

pub fn write_registry(e: &Env, id: &Address) {
    let key = DataKey::Registry;
    e.storage().instance().set(&key, id);
}
