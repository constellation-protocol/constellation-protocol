use soroban_sdk::{contract, contractimpl, panic_with_error,contracttype, Address, Env};
use crate::storage::admin::{ has_administrator, write_administrator };
use crate::storage::module::{
    remove_module as _remove_module, 
    write_module
};
use crate::validation::require_administrator;
use crate::error::Error;
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
    pub fn add_module(e: Env,  module_id: Address) -> Result<(), Error> {
        require_administrator(&e)?;
        write_module(&e, module_id);
        Ok(())
    }
    pub fn  remove_module(e: Env, module_id: Address) -> Result<(), Error> {
        require_administrator(&e)?;
        _remove_module(&e, module_id);
        Ok(())
    }
}
