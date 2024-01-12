use soroban_sdk::{Address, Env, String, Vec};
use crate::types::CreateConstellationTokenArgs;
pub(crate) mod constellation_token {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}
pub(crate) fn initialize_token(
    e: &Env,
    address: &Address,
    decimal: u32,
    name: String,
    symbol: String,
    admin: Address,
    manager: Address,
    components: Vec<Address>,
    amounts: Vec<i128>,
) {
    let ct = constellation_token::Client::new(e, address);

    let client = constellation_token::Client::new(&e, &address);
    client.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );
}
