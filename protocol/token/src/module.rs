use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;
 
// pub fn read_module(e: &Env, id: &Address) -> Option<Address> {
//     let key = DataKey::Module(id);
//     e.storage().instance().get(&key)
// }

// pub fn write_module(e: &Env, id: &Address) {
//     let key = DataKey::Module;
//     e.storage().instance().set(&key, id);
// }

// pub fn remove_module(e: &Env, id: &Address) {
//     let key = DataKey::Module(id);
//     e.storage().instance().remove(&key);
// }