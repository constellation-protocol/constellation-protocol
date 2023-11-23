#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, symbol_short, token, Address, Env,
    Symbol,
};

mod constellation_token {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}

#[contract]
struct ConstellationMinterBurner;

#[contractimpl]
impl ConstellationMinterBurner {
    pub fn mint(env: Env, from: Address, to: Address, token_address: Address, amount: i128) {
        // Minter account needs to authorize
        from.require_auth();

        let ctoken = constellation_token::Client::new(&env, &token_address);

        ctoken.mint(&to, &amount);
    }
}
