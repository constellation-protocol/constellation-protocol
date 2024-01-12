use crate::storage::read_factory;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

pub(crate) mod constellation_factory {
    soroban_sdk::contractimport! {
        file = "../../target/wasm32-unknown-unknown/release/constellation_factory.wasm"
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn create(
    e: &Env,
    decimal: u32,
    name: String,
    symbol: String,
    admin: &Address,
    manager: Address,
    components: Vec<Address>,
    amounts: Vec<i128>,
    deployer: Address,
    wasm_hash: BytesN<32>,
    salt: BytesN<32>,
) -> Address {
    let address = read_factory(&e);
    let factory = constellation_factory::Client::new(e, &address);
    let adddress: Address = factory.create(
        &decimal,
        &name,
        &symbol,
        &admin,
        &manager,
        &components,
        &amounts,
        &deployer,
        &wasm_hash,
        &salt,
    );

    adddress
}
