use super::keys::DataKey;
use super::types::{Component, PERSISTENT_LEDGER_LIFE, PERSISTENT_LEDGER_TTL_THRESHOLD};
use crate::error::{check_zero_or_negative_amount, Error};
use soroban_sdk::{contracttype, panic_with_error, Address, Env, Vec};
extern crate alloc;
use alloc::vec;

pub fn write_components(
    e: &Env,
    components_address: &Vec<Address>,
    units: &Vec<i128>,
) -> Vec<Component> {
    if components_address.len() != units.len() {
        panic_with_error!(e, Error::ComponentsAmountsLengthMismatch);
    }

    if components_address.len() == 0 {
        panic_with_error!(e, Error::ZeroComponents);
    }
    let mut components: Vec<Component> = Vec::new(e);

    for i in 0..components_address.len() {
        let address = components_address
            .get(i)
            .unwrap_or_else(|| panic_with_error!(&e, Error::IndexUnwrapError));
        let unit = units
            .get(i)
            .unwrap_or_else(|| panic_with_error!(e, Error::IndexUnwrapError));

        check_zero_or_negative_amount(e, unit);

        components.push_back(Component { address, unit });
    }
    let key = DataKey::Components;
    e.storage().persistent().set(&key, &components);

    e.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LEDGER_TTL_THRESHOLD,
        PERSISTENT_LEDGER_LIFE,
    );
    components
}

pub fn read_components(e: &Env) -> Vec<Component> {
    let key = DataKey::Components;
    if let Some(components) = e
        .storage()
        .persistent()
        .get::<DataKey, Vec<Component>>(&key)
    {
        e.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_LEDGER_TTL_THRESHOLD,
            PERSISTENT_LEDGER_LIFE,
        );
        components
    } else {
        Vec::new(e)
    }
}
