// use super::registry::get_adapter_id;
use crate::error::Error;
use crate::registry::is_registered_module;
use crate::storage::module::is_registered;
use crate::storage::{admin::read_administrator, manager::read_manager, registry::read_registry};
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env};

pub fn require_administrator(e: &Env) -> Result<Address, Error> {
    let admin = match read_administrator(e) {
        Some(admin) => admin,
        None => return Err(Error::RequiresAdministrator),
    };
    Ok(admin)
}

pub fn require_registry(e: &Env) -> Result<Address, Error> {
    let registry_id = match read_registry(e) {
        Some(registry_id) => registry_id,
        None => return Err(Error::RequiresRegistry),
    };
    Ok(registry_id)
}

pub fn require_manager(e: &Env) -> Result<Address, Error> {
    let manage_id = match read_manager(&e) {
        Some(manage_id) => manage_id,
        None => return Err(Error::RequiresManage),
    };
    Ok(manage_id)
}

pub fn assert_token_registered_module(e: &Env, module_id: &Address) -> Result<(), Error> { 
 
    if is_registered(e, &module_id) == false { 
       return Err(Error::RequiresTokenRegisteredModule);
    }
    Ok(())
}

pub fn assert_registered_module(
    e: &Env,
    module_id: &Address,
    registry_id: &Address,
) -> Result<(), Error> {
    if is_registered_module(e, module_id, registry_id) == false {
        return Err(Error::ModuleNotInRegistery);
    };
    Ok(())
}