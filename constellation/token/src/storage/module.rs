use super::keys::DataKey;
use crate::error::Error;
use soroban_sdk::{Address, Env};

pub fn read_module(e: &Env, id: &Address) -> Option<Address> {
    let key = DataKey::Module(id.clone());
    e.storage().instance().get(&key)
}

pub fn write_module(e: &Env, id: &Address) {
    let key = DataKey::Module(id.clone());
    e.storage().instance().set(&key, &id);
}

pub fn remove_module(e: &Env, id: &Address) {
    let key: DataKey = DataKey::Module(id.clone());
    e.storage().instance().remove(&key);
}

pub fn is_registered(e: &Env, id: &Address) -> bool {
    match read_module(e, id) {
        Some(_) => true,
        None => false,
    }
}
