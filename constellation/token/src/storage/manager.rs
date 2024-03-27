use soroban_sdk::{Address, Env};
use super::keys::DataKey;

pub fn has_manager(e: &Env) -> bool {
    let key = DataKey::Manager;
    e.storage().instance().has(&key)
}

pub fn read_manager(e: &Env) -> Option<Address> {
    let key = DataKey::Manager;
    e.storage().instance().get(&key)
}

pub fn write_manager(e: &Env, id: &Address) {
    let key = DataKey::Manager;
    e.storage().instance().set(&key, id);
}
