use soroban_sdk::{Address, Env, String, Symbol, Val, Vec};

pub(crate) mod registry {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_registry.wasm"
    );
}

pub(crate) fn is_registered_module(e: &Env, module_id: &Address, registry_id: &Address) -> bool {
    let client = registry::Client::new(&e, &registry_id);
    client.is_registered_module(&module_id)
}
