use super::registry::get_adapter_id;
use crate::{error::Error, storage::admin::read_administrator};
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env};

pub fn require_administrator(e: &Env) -> Result<(), Error> {
    match read_administrator(e) {
        Some(admin) => admin.require_auth(),
        None => return Err(Error::RequiresAdmin),
    }
    Ok(())
}

// pub fn panic_unregistered_adapter(e: &Env, module_id: &Address, exchange_id: &Address) -> Address {
//     let adapter = match get_adapter_id(e, exchange_id) {
//         Some(adapter) => adapter,
//         None => panic_with_error!(e, Error::UnregisteredAdapter),
//     };
//     adapter
// }
