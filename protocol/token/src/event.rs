use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redeem {
    spender: Address,
    from: Address,
    amount: i128,
}

pub(crate) fn emit_redeem(e: &Env, spender: Address, from: Address, amount: i128) {
    e.events().publish(
        ("ConstellationToken", symbol_short!("redeem")),
        Redeem {
            spender,
            from,
            amount,
        },
    );
}
