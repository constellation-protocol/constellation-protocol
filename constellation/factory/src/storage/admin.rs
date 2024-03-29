use soroban_sdk::{Address, Env};

use super::DataKey;

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    e.storage().instance().get(&key)
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}
