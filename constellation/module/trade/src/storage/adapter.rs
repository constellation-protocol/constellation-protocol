
use soroban_sdk::{panic_with_error, Address, Env};
use crate::error::Error;
use super::keys::DataKey;

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

pub fn read_or_panic_unregistered_adapter(e: &Env, exchange_id: &Address) -> Address {
    let adapter = match read_adapter(e, exchange_id) {
        Some(adapter) => adapter,
        None => panic_with_error!(e, Error::UnregisteredAdapter),
    };
    adapter
} 