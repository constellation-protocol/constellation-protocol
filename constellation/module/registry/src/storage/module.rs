
use soroban_sdk::{panic_with_error, Address, Env};
use crate::error::Error;
use super::keys::DataKey;

pub fn read_module(e: &Env, exchange_id: &Address) -> Option<Address> {
    let key = DataKey::Module(exchange_id.clone());
    e.storage().instance().get(&key)
}

pub fn write_module(e: &Env, exchange_id: Address, module_id: Address) {
    let key = DataKey::Module(exchange_id);
    e.storage().instance().set(&key, &module_id);
}

pub fn remove_module(e: &Env, exchange_id: Address) {
    let key: DataKey = DataKey::Module(exchange_id);
    e.storage().instance().remove(&key);
}

pub fn read_or_panic_unregistered_module(e: &Env, exchange_id: &Address) -> Address {
    let module = match read_module(e, exchange_id) {
        Some(module) => module,
        None => panic_with_error!(e, Error::UnregisteredModule),
    };
    module
} 