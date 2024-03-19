use crate::traits::adapter::{self, CallData};
use soroban_sdk::{Address, Env, String, Symbol, Vec};

pub(crate) mod constellation_token {
    soroban_sdk::contractimport!(
        file = "../../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}

/// Initializes the deployed constellation token
///
/// # Arguments
///
/// - `e` The runtime environment.
/// - `token_address` Address of deployed constellation token
/// - `decimal` Token decimal
/// - `name` Name of token
/// - `symbol` Symbol of token
/// - `admin` Token administrator
/// - `manager` Manages constellation token components and rebalancing
/// - `components` Component tokens of this token
/// - `amounts` Amounts of each componet token required to mint constellation token
pub(crate) fn invoke(
    e: &Env,
    constellation_token_id: &Address,
    target_exchange_id: &Address,
    call_data: CallData,
) {
    let client = constellation_token::Client::new(&e, &constellation_token_id);
    client.invoke(
        &e.current_contract_address(),
        &target_exchange_id,
        &call_data.function,
        &call_data.data,
    );
}
