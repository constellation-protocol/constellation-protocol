use soroban_sdk::{auth::InvokerContractAuthEntry, Address, Env, Symbol, Val, Vec};

pub mod constellation_token {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}
pub use constellation_token::Component;

pub(crate) fn mint(e: &Env, to: &Address, amount: i128, constellation_token_address: &Address) {
    let ctoken: constellation_token::Client<'_> =
        constellation_token::Client::new(&e, constellation_token_address);
    ctoken.mint(to, &amount);
}

pub(crate) fn redeem(
    e: &Env,
    from: &Address,
    to: &Address,
    amount: i128,
    constellation_token_address: &Address,
) {
    let ctoken = constellation_token::Client::new(&e, constellation_token_address);
    ctoken.burn_from(&e.current_contract_address(), from, &amount);
    ctoken.redeem(to, &amount);
}

pub(crate) fn get_components(e: &Env, constellation_token_address: &Address) -> Vec<Component> {
    let ctoken = constellation_token::Client::new(&e, constellation_token_address);
    ctoken.get_components()
}

pub(crate) fn invoke(
    e: &Env,
    constellation_token_id: &Address,
    target_contract_id: &Address,
    call_data: &(Symbol, Vec<Val>),
    auth_entries: &Vec<InvokerContractAuthEntry>,
) {
    let client = constellation_token::Client::new(&e, &constellation_token_id);
    client.invoke(
        &e.current_contract_address(),
        target_contract_id,
        call_data,
        auth_entries,
    );
}
