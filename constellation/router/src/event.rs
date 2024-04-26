use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Initialize {
    factory: Address,
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintExactConstellation {
    to: Address,
    amount: i128,
    refund: i128,
}

/// Emits initialize contract even
pub(crate) fn initialize(e: &Env, factory: Address) {
    let topics = (Symbol::new(e, "intialize"), e.current_contract_address());
    e.events().publish(topics, Initialize { factory });
}

pub(crate) fn mint_exact_constellation(e: &Env, to: Address, amount: i128, refund: i128) {
    let topics = (Symbol::new(e, "mint_exact_constellation"), e.current_contract_address());
    e.events().publish(topics, MintExactConstellation { to, amount, refund });
}
