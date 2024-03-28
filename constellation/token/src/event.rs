use soroban_sdk::{contracttype, symbol_short, Address, Env, IntoVal, String, Symbol, Vec};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

use crate::storage::types::Component;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redeem {
    spender: Address,
    from: Address,
    amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Initialize {
    components: Vec<Component>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetManager {
    old_manager: Address,
    new_manager: Address,
}

pub(crate) fn redeem(e: &Env, spender: Address, from: Address, amount: i128) {
    let topics = (Symbol::new(e, "redeem"),);
    e.events().publish(
        topics,
        Redeem {
            spender,
            from,
            amount,
        },
    );
}

pub(crate) fn set_manager(e: &Env, old_manager: Address, new_manager: Address) {
    let topics = (Symbol::new(e, "set_manager"),);
    e.events().publish(
        topics,
        SetManager {
            old_manager,
            new_manager,
        },
    );
}

pub(crate) fn initialize(e: &Env, components: Vec<Component>) {
    let topics = (Symbol::new(e, "intialize"), e.current_contract_address());
    e.events().publish(topics, Initialize { components });
}
