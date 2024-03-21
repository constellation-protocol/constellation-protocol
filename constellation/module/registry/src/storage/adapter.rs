use super::keys::DataKey;
use super::module::read_or_panic_unregistered_module;
use crate::error::Error;
use soroban_sdk::{panic_with_error, Address, Env};
pub fn read_adapter(e: &Env, module_id: Address, target_id: Address) -> Option<Address> {
    let key = DataKey::Adapter(module_id, target_id.clone());
    e.storage().instance().get(&key)
}

pub fn write_adapter(e: &Env, module_id: Address, target_id: Address, adapter_id: Address) {
    read_or_panic_unregistered_module(e, &module_id);
    let key = DataKey::Adapter(module_id, target_id.clone());
    e.storage().instance().set(&key, &adapter_id);
}

pub fn remove_adapter(e: &Env, module_id: Address, target_id: Address) {
    let key: DataKey = DataKey::Adapter(module_id, target_id);
    e.storage().instance().remove(&key);
}

pub fn has_adapter(e: &Env, module_id: Address, target_id: &Address) -> bool {
    match read_adapter(e, module_id, target_id.clone()) {
        Some(adapter_id) => true,
        None => false,
    }
}
