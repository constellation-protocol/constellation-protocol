use crate::error::{check_zero_or_negative_amount, Error};
use crate::types::Component;
use crate::types::{DataKey, COMPONENTS_BUMP_AMOUNT, COMPONENTS_LIFETIME_THRESHOLD};
use soroban_sdk::{contracttype, panic_with_error, Address, Env, Vec};
extern crate alloc;
use alloc::vec;

pub fn write_components(e: &Env, components_address: Vec<Address>, amounts: Vec<i128>) {
    if components_address.len() != amounts.len() {
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
        let amount = amounts
            .get(i)
            .unwrap_or_else(|| panic_with_error!(e, Error::IndexUnwrapError));

        check_zero_or_negative_amount(e, amount);

        components.push_back(Component { address, amount });
    }
    let key = DataKey::Components;
    e.storage().persistent().set(&key, &components);

    e.storage().persistent().extend_ttl(
        &key,
        COMPONENTS_LIFETIME_THRESHOLD,
        COMPONENTS_BUMP_AMOUNT,
    );
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
            COMPONENTS_LIFETIME_THRESHOLD,
            COMPONENTS_BUMP_AMOUNT,
        );
        components
    } else {
        Vec::new(e)
    }
}
