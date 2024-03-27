use super::keys::DataKey;
use crate::error::Error;
use soroban_sdk::{Address, Env};

pub fn read_module(e: &Env, id: Address) -> Option<bool> {
    let key = DataKey::Module(id);
    e.storage().instance().get(&key)
}

pub fn write_module(e: &Env, id: Address) {
    let key = DataKey::Module(id);
    e.storage().instance().set(&key, &true);
}

pub fn remove_module(e: &Env, id: Address) {
    let key: DataKey = DataKey::Module(id);
    e.storage().instance().remove(&key);
}

pub fn is_registered(e: &Env, id: Address) -> bool {
    read_module(e, id).unwrap_or(false)
}
