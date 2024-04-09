use soroban_sdk::{Address, Env};

pub mod constellation_token {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}

pub(crate) fn mint(e: &Env, to: Address, amount: i128, constellation_token_address: Address) {
    let ctoken = constellation_token::Client::new(&e, &constellation_token_address);
    ctoken.mint(&to, &amount);
}

pub(crate) fn redeem(e: &Env, from: Address, amount: i128, constellation_token_address: Address) {
    let ctoken = constellation_token::Client::new(&e, &constellation_token_address);
    ctoken.burn_from(&e.current_contract_address(), &from, &amount);
    ctoken.redeem(&&from, &amount);
}
