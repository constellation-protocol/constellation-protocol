use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Initialize {
    factory: Address,
}

pub(crate) fn initialize(e: &Env, factory: Address) {
    let topics = (Symbol::new(e, "intialize"), e.current_contract_address());
    e.events().publish(topics, Initialize { factory });
}
