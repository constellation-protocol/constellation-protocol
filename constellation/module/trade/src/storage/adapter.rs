
use soroban_sdk::{panic_with_error, Address, Env};
use crate::error::Error;
use super::keys::DataKey;

pub fn read_adapter(e: &Env, exchange_id: &Address) -> Option<bool> {
    let key = DataKey::Adapter(exchange_id.clone());
    e.storage().instance().get(&key)
}

pub fn write_adapter(e: &Env, exchange_id: Address, adapter_id: Address) {
    let key = DataKey::Adapter(exchange_id);
    e.storage().instance().set(&key, &adapter_id);
}

pub fn remove_adapter(e: &Env, exchange_id: Address) {
    let key: DataKey = DataKey::Adapter(exchange_id);
    e.storage().instance().remove(&key);
}

pub fn panic_unregistered_adapter(e: &Env, exchange_id: &Address) {
    if is_registered(e, exchange_id) == false {
        panic_with_error!(e, Error::UnregisteredAdapter);
    }
}

pub fn is_registered(e: &Env, exchange_id: &Address) -> bool {
    read_adapter(e, exchange_id).unwrap_or(false)
}

