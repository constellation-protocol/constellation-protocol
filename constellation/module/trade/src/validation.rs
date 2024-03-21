use super::registry::get_adapter_id;
use crate::registry::get_adapter_id;
use crate::{error::Error, storage::admin::read_administrator};
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env};

pub fn require_administrator(e: &Env) -> Result<(), Error> {
    match read_administrator(e) {
        Some(admin) => admin.require_auth(),
        None => return Err(Error::RequiresAdmin),
    }
    Ok(())
}

pub fn require_registry(e: &Env) -> Result<Address, Error> {
    let registry_id = match read_registry(e) {
        Some(registry_id) => registry_id,
        None => return Err(Error::RequiresRegistry),
    };
    Ok(registry_id)
}

pub fn require_adapter(
    e: &Env,
    registry_id: &Address,
    exchange_id: &Address,
) -> Result<Address, Error> {
    let adapter_id = match get_adapter_id(e, registry_id, exchange_id) {
        Some(adapter_id) => adapter_id,
        None => return Err(Error::RequiresExchangeAdapter),
    };
    Ok(adapter_id)
} 