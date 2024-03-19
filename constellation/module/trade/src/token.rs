use soroban_sdk::{Address, Env, String, Vec};
pub(crate) mod constellation_token {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
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
    method: &Address,
    call_data: CallData, 
) {
    let client = constellation_token::Client::new(&e, &token_address);
    client.invoke(
        &method,
        &call_data
    );
}
