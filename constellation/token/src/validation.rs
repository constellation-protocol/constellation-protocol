use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env};
use super::registry::get_adapter_id;
use crate::error::Error;
use crate::storage::{admin::read_administrator,manager::read_manager
    , registry::read_registry};

pub fn require_administrator(e: &Env) -> Result<Address, Error> {
    let admin = match read_administrator(e) {
        Some(admin) => {
            admin.require_auth();
            admin
        }
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
        Some(manage_id) => {
            manage_id.require_auth();
            manage_id
        },
        None => return Err(Error::RequiresManage),
    };
    Ok(manage_id)
}
