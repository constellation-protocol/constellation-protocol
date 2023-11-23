#![cfg(test)]
extern crate std;

use std::println;

use crate::contract::ConstellationTokenClient;

use crate::component::read_components;
use crate::token_interface_storage::admin::read_administrator;
use soroban_sdk::{contracttype, Address, String};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, BytesN, Env, IntoVal, Vec,
};

fn create_constellation_token(
    e: &Env,
    decimal: u32,
    components: Vec<Address>,
    amounts: Vec<i128>,
    name: String,
    symbol: String,
    admin: Address,
    manager: Address,
) -> ConstellationTokenClient {
    let contract_id = &e.register_contract(None, crate::contract::ConstellationToken);
    let ct: ConstellationTokenClient<'_> = ConstellationTokenClient::new(e, contract_id);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );
    ct
}

#[test]
fn test_initialize() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::random(&e),
        Address::random(&e),
        Address::random(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::random(&e);
    let manager = Address::random(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(
        &e,
        decimal,
        components.clone(),
        amounts.clone(),
        name,
        symbol,
        admin.clone(),
        manager.clone(),
    );

    assert_eq!(ct.admin(), admin);
    assert_eq!(ct.components().len(), 3);
}
