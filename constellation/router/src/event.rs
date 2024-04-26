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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedeemInto {
    to: Address,
    redeem_token: Address,
    constellation_token: Address,
    amount: i128,
}

/// Emits initialize contract even
pub(crate) fn initialize(e: &Env, factory: Address) {
    let topics = (Symbol::new(e, "intialize"), e.current_contract_address());
    e.events().publish(topics, Initialize { factory });
}

pub(crate) fn mint_exact_constellation(e: &Env, to: Address, amount: i128, refund: i128) {
    let topics = (
        Symbol::new(e, "mint_exact_constellation"),
        e.current_contract_address(),
    );
    e.events()
        .publish(topics, MintExactConstellation { to, amount, refund });
}

pub(crate) fn redeem_into(
    e: &Env,
    to: Address,
    redeem_token: Address,
    constellation_token: Address,
    amount: i128,
) {
    let topics = (Symbol::new(e, "redeem_into"), e.current_contract_address());
    e.events().publish(
        topics,
        RedeemInto {
            to,
            redeem_token,
            constellation_token,
            amount,
        },
    );
}
