use crate::storage::adapter::{read_adapter, remove_adapter as _remove_adapter, write_adapter};
use crate::storage::admin::{has_administrator, write_administrator};
use crate::storage::module::{read_module, remove_module as _remove_module, write_module};
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env};

use crate::error::Error;
use crate::validation::require_administrator;
#[contract]
pub struct Registry {}

#[contractimpl]
impl Registry {
    pub fn initialize(e: Env, id: Address) {
        if has_administrator(&e) {
            panic_with_error!(&e, Error::AlreadyInitalized);
        }
        write_administrator(&e, &id);
    }
    pub fn add_module(e: Env, module_id: Address) -> Result<(), Error> {
        require_administrator(&e)?;
        write_module(&e, module_id);
        Ok(())
    }
    pub fn remove_module(e: Env, module_id: Address) -> Result<(), Error> {
        require_administrator(&e)?;
        _remove_module(&e, module_id);
        Ok(())
    }

    pub fn is_registered_module(e: Env, module_id: Address) -> bool {
        match read_module(&e, &module_id) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn add_adapter(
        e: Env,
        module_id: Address,
        target_id: Address,
        adapter_id: Address,
    ) -> Result<(), Error> {
        require_administrator(&e)?;
        write_adapter(&e, module_id, target_id, adapter_id);
        Ok(())
    }
    pub fn remove_adapter(e: Env, module_id: Address, target_id: Address) -> Result<(), Error> {
        require_administrator(&e)?;
        _remove_adapter(&e, module_id, target_id);
        Ok(())
    }

    pub fn get_adapter_id(e: Env, module_id: Address, target_id: Address) -> Option<Address> {
        read_adapter(&e, module_id, target_id)
    }
}
