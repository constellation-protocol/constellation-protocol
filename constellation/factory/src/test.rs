#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, BytesN, Env, InvokeError, Val,
};
// use soroban_env_common
use crate::token::constellation_token;
use crate::{
    contract::{Factory, FactoryClient},
    error::Error,
};
use soroban_sdk::IntoVal;

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_factory<'a>(e: &Env) -> FactoryClient<'a> {
    let contract_id = &e.register_contract(None, Factory {});
    let ct: FactoryClient<'_> = FactoryClient::new(e, contract_id);
    ct
}

fn create_constellation_token<'a>(e: &Env) -> (constellation_token::Client<'a>, Address) {
    let contract_id = &e.register_contract_wasm(None, constellation_token::WASM);
    let ct: constellation_token::Client<'_> = constellation_token::Client::new(e, contract_id);
    (ct, contract_id.clone())
}

#[test]
pub fn create_constellation_token_fails_with_exceeds_max_components() {
    let e = Env::default();
    e.mock_all_auths();
    let mut user = Address::generate(&e);

    let token1 = create_token_contract(&e, &user);
    let token2 = create_token_contract(&e, &user);
    let token3 = create_token_contract(&e, &user);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &50000000000);
    token2.mint(&user1, &20000000000);
    token3.mint(&user1, &20000000000);
    let components = vec![
        &e,
        token1.address.clone(),
        token2.address.clone(),
        token3.address.clone(),
    ];

    let amounts = vec![&e, 10000000000, 20000000000, 20000000000];
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);

    let factory = create_factory(&e);
    factory.initialize(&user);
    factory.set_max_components(&2u32);
    let result = factory.try_create(
        &6u32,
        &"USDC".into_val(&e),
        &"USDC".into_val(&e),
        &user,
        &user,
        &components,
        &amounts,
        &factory.address,
        &wasm_hash,
        &wasm_hash,
    );

    assert_eq!(result, Err(Ok(Error::ExceedsMaxComponents)));
}

#[test]
pub fn create_constellation_token_succeeds() {
    let e = Env::default();
    e.mock_all_auths();
    let mut user = Address::generate(&e);

    let token1 = create_token_contract(&e, &user);
    let token2 = create_token_contract(&e, &user);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &50000000000);
    token2.mint(&user1, &20000000000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 10000000000, 20000000000];
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);

    let factory = create_factory(&e);
    let result = factory.create(
        &6u32,
        &"USDC".into_val(&e),
        &"USDC".into_val(&e),
        &user,
        &user,
        &components,
        &amounts,
        &factory.address,
        &wasm_hash,
        &wasm_hash,
    );

    let constellation_tokens = factory.get_token_list();
    assert_eq!(result, constellation_tokens.get(0).unwrap());
}

#[test]
pub fn set_max_components_fails_with_requires_administrator() {
    let e = Env::default();
    e.mock_all_auths();

    let factory = create_factory(&e);
    let result = factory.try_set_max_components(&2u32);

    assert_eq!(result, Err(Ok(Error::RequiresAdministrator)));
}

#[test]
pub fn set_max_components_succeeds() {
    let e = Env::default();
    e.mock_all_auths();
    let user = Address::generate(&e);

    let factory = create_factory(&e);
    factory.initialize(&user);
    let result = factory.try_set_max_components(&2u32);

    assert_eq!(result, Ok(Ok(())));
}

#[test]
pub fn initialize_fails_with_already_initialized() {
    let e = Env::default();
    e.mock_all_auths();
    let user = Address::generate(&e);
    let factory = create_factory(&e);
    factory.initialize(&user);
    let result = factory.try_initialize(&user);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
pub fn initialize_succeeds() {
    let e = Env::default();
    e.mock_all_auths();
    let user = Address::generate(&e);
    let factory = create_factory(&e);
    let result = factory.try_initialize(&user);
    assert_eq!(result, Ok(Ok(())));
}
