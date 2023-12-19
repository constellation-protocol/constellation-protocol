#![cfg(test)]
extern crate std;

use crate::contract::ConstellationTokenClient;
use std::println;

use super::error::Error;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, BytesN, Env, IntoVal, Vec,
};
use soroban_sdk::{Address, String};

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
}
fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_constellation_token<'a>(e: &Env) -> ConstellationTokenClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::ConstellationToken {});
    let ct: ConstellationTokenClient<'_> = ConstellationTokenClient::new(e, contract_id);
    ct
}

#[test] 
fn test_initialize_should_panic_with_already_initalized() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::AlreadyInitalized)));
    assert_eq!(ct.admin(), admin);
    assert_eq!(ct.manager(), manager);
    assert_eq!(ct.components().len(), 3);
}

#[test]
fn test_initialize_should_panic_with_components_amounts_length_mismatch() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ComponentsAmountsLengthMismatch)));
}

#[test]
fn test_initialize_should_panic_with_zero_components() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![&e];
    let amounts: Vec<i128> = vec![&e];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ZeroComponents)));
}

#[test]
fn test_initialize_should_panic_with_zero_or_negative_amount_not_allowed_1() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, -1];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ZeroOrNegativeAmountNotAllowed))); 
}

#[test]
fn test_initialize_should_panic_with_zero_or_negative_amount_not_allowed_2() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 0];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    let res = ct.try_initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(res, Err(Ok(Error::ZeroOrNegativeAmountNotAllowed))); 
}

#[test]
fn test_initialize_successful() {
    let e = Env::default();
    e.mock_all_auths();
    let components = vec![
        &e,
        Address::generate(&e),
        Address::generate(&e),
        Address::generate(&e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );

    assert_eq!(ct.admin(), admin);
    assert_eq!(ct.manager(), manager);
    assert_eq!(ct.components().len(), 3);
}


// todo! Test mint 