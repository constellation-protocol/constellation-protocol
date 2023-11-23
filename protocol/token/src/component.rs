use crate::error::{ensure_equal_lengths, ensure_none_zero};
use crate::types::Component;
use crate::types::{DataKey, COMPONENTS_BUMP_AMOUNT, COMPONENTS_LIFETIME_THRESHOLD};
use alloc::vec;
use soroban_sdk::{contracttype, Address, Env, Vec};
extern crate alloc;

pub fn write_components(e: &Env, components_address: Vec<Address>, amounts: Vec<i128>) {
    ensure_equal_lengths(
        components_address.len(),
        amounts.len(),
        "components",
        "amounts",
    );
    ensure_none_zero(components_address.len(), "ZERO_COMPONENTS");

    let mut components: Vec<Component> = Vec::new(e);

    for i in 0..components_address.len() {
        match components_address.get(i) {
            Some(address) => match amounts.get(i) {
                Some(amount) => {
                    if amount == 0 {
                        panic!("amount zero");
                    }
                    components.push_back(Component { address, amount });
                }
                None => panic!("amount zero"),
            },
            None => panic!("none address"),
        }
    }
    let key = DataKey::Components;

    e.storage().persistent().set(&key, &components);
    e.storage()
        .persistent()
        .bump(&key, COMPONENTS_LIFETIME_THRESHOLD, COMPONENTS_BUMP_AMOUNT);
}

pub fn read_components(e: &Env) -> Vec<Component> {
    let key = DataKey::Components;
    if let Some(components) = e
        .storage()
        .persistent()
        .get::<DataKey, Vec<Component>>(&key)
    {
        e.storage()
            .persistent()
            .bump(&key, COMPONENTS_LIFETIME_THRESHOLD, COMPONENTS_BUMP_AMOUNT);
        components
    } else {
        Vec::new(e)
    }
}
