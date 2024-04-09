use soroban_sdk::{Address, Env};

use super::keys::DataKey;

pub fn has_router(e: &Env) -> bool {
    let key = DataKey::Router;
    e.storage().instance().has(&key)
}

pub fn read_router(e: &Env) -> Option<Address> {
    let key = DataKey::Router;
    e.storage().instance().get(&key)
}

pub fn write_router(e: &Env, id: &Address) {
    let key = DataKey::Router;
    e.storage().instance().set(&key, id);
}
