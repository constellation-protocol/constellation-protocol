
use soroban_sdk::{contract, contractimpl, panic_with_error,contracttype, Address, Env};

use crate::{error::Error, storage::admin::read_administrator};
 
pub fn require_administrator(e: &Env) -> Result<(), Error> {
    match read_administrator (e) {
        Some(admin) => admin.require_auth(),
        None => return Err(Error::RequiresAdmin),
    } 
    Ok(())
}