use super::keys::DataKey;
use crate::error::Error;
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_adapter(e: &Env, exchange_id: &Address) -> Option<Address> {
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
