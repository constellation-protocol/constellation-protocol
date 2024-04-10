use super::keys::DataKey;
use super::types::{Component, PERSISTENT_LEDGER_LIFE, PERSISTENT_LEDGER_TTL_THRESHOLD};
use crate::error::{check_zero_or_negative_amount, Error};
use soroban_sdk::{contracttype, Map, panic_with_error, Address, Env, Vec};
extern crate alloc;
use alloc::vec;

pub fn write_components(
    e: &Env,
    components_address: &Vec<Address>,
    units: &Vec<i128>,
)  {
    if components_address.len() != units.len() {
        panic_with_error!(e, Error::ComponentsAmountsLengthMismatch);
    }

    if components_address.len() == 0 {
        panic_with_error!(e, Error::ZeroComponents);
    }

    let mut store = _read_store(e);
    for (i, address) in components_address.iter().enumerate() {
 
        let unit = units
            .get(i as u32 )
            .unwrap_or_else(|| panic_with_error!(e, Error::IndexUnwrapError));

        check_zero_or_negative_amount(e, unit);
        store.set(address.clone(), Component { address, unit }); 
    } 
  _write_store(e, &store);
}

pub fn read_components_list(e: &Env) -> Vec<Component>{
    Vec::from(_read_store(e).values()) 
}

pub fn read_components(e: &Env) -> Map<Address, Component>{
    _read_store(e)
}

pub fn read_component(e: &Env, address: Address) -> Option<Component> {
     _read_store(e).get(address)
}

pub fn write_component(e: &Env, address: Address, component: Component) {
     let mut store  = _read_store(&e);
     store.set(address,component );
     _write_store(e, &store);
}

pub fn remove_component(e: &Env, address: Address) {
    let mut store  = _read_store(&e);
    store.remove(address);
    _write_store(e, &store);
}

fn _read_store(e: &Env) -> Map<Address, Component> {
    let key = DataKey::Components;
 
    let store = match  e
    .storage()
    .persistent()
    .get(&key) {
        Some(store) => store,
        None => Map::new(e)
    };
    store 
}

fn _write_store(e: &Env, store: &Map<Address, Component>) {
    let key = DataKey::Components;
    e.storage().persistent().set(&key, store);
}

fn _extend_ttl(e: &Env) {
    let key = DataKey::Components;
    e.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LEDGER_TTL_THRESHOLD,
        PERSISTENT_LEDGER_LIFE,
    );
}