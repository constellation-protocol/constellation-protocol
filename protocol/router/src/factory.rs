use crate::storage::read_factory;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

pub(crate) mod constellation_factory {
    soroban_sdk::contractimport! {
        file = "../../target/wasm32-unknown-unknown/release/constellation_factory.wasm"
    }
}

/// Makes a cross contract call to the factory contract to deploy a constellation token 
///  Returns the address of the deployed constellation token
///
///  # Arguments
///
/// - `e` The runtime environment.
/// - `decimal` Token decimal
/// - `name` Name of token
/// - `symbol` Symbol of token
/// - `admin` Token administrator
/// - `manager` Manages constellation token components and rebalancing
/// - `components` Component tokens of this token
/// - `amounts` Amounts of each componet token required to mint constellation token
/// - `factory_address` Address of factory contract 
/// - `wasm_hash` constellation token wasm has
/// - `salt` Unique salt
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
    factory_address: Address,
    wasm_hash: BytesN<32>,
    salt: BytesN<32>,
) -> Address {
    let factory = constellation_factory::Client::new(e, &factory_address);
    let constellation_token_address: Address = factory.create(
        &decimal,
        &name,
        &symbol,
        &admin,
        &manager,
        &components,
        &amounts,
        &factory_address,
        &wasm_hash,
        &salt,
    );

    constellation_token_address
}
