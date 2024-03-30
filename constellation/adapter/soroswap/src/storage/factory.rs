use soroban_sdk::{Address, Env};

use super::keys::DataKey;

pub fn has_factory(e: &Env) -> bool {
    let key = DataKey::Factory;
    e.storage().instance().has(&key)
}

pub fn read_factory(e: &Env) -> Option<Address> {
    let key = DataKey::Factory;
    e.storage().instance().get(&key)
}

pub fn write_factory(e: &Env, id: &Address) {
    let key = DataKey::Factory;
    e.storage().instance().set(&key, id);
}
