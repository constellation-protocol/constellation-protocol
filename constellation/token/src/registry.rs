use soroban_sdk::{Address, Env, String, Symbol, Val, Vec};

pub(crate) mod registry {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_module_registry.wasm"
    );
}

/// Invokes the invoke function of the constellation token to trade / exchange tokens
///
/// # Arguments
///
/// - `e` The runtime environment.
/// - `constellation_token_id` Target constellation token id
/// - `target_exchange_id` Target exchange identifier
/// - `function` Name of function to invoke on target exchange
/// - `data` Function arguments
pub(crate) fn get_adapter_id(
    e: &Env,
    registry_id: &Address,
    target_exchange_id: &Address,
) -> Option<Address> {
    let client = registry::Client::new(&e, &registry_id);
    client.get_adapter_id(&e.current_contract_address(), &target_exchange_id)
}

pub(crate) fn is_registered_module(e: &Env, module_id: &Address, registry_id: &Address,) -> bool  {
    let client = registry::Client::new(&e, &registry_id);
    client.is_registered_module(&module_id)
}