use super::keys::DataKey;
use crate::error::Error;
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_module(e: &Env, module_id: &Address) -> Option<Address> {
    let key = DataKey::Module(module_id.clone());
    e.storage().instance().get(&key)
}

pub fn write_module(e: &Env, module_id: Address) {
    let key = DataKey::Module(module_id.clone());
    e.storage().instance().set(&key, &module_id);
}

pub fn remove_module(e: &Env, module_id: Address) {
    let key: DataKey = DataKey::Module(module_id);
    e.storage().instance().remove(&key);
}

pub fn read_or_panic_unregistered_module(e: &Env, module_id: &Address) -> Address {
    let module = match read_module(e, module_id) {
        Some(module) => module,
        None => panic_with_error!(e, Error::UnregisteredModule),
    };
    module
}
