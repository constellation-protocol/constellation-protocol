
use soroban_sdk::{Address, Env};

use super::keys::DataKey;

pub fn read_adapter(e: &Env, exchange_id: Address) -> Option<bool> {
    let key = DataKey::Adapter(exchange_id);
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

pub fn is_registered(e: &Env, exchange_id: Address) -> bool {
    read_adapter(e, exchange_id).unwrap_or(false)
}
