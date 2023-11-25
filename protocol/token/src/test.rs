#![cfg(test)]
extern crate std;

use crate::contract::ConstellationTokenClient;
use std::println;

use crate::component::read_components;
use crate::token;
use crate::token_interface_storage::admin::read_administrator;
use soroban_sdk::{contracttype, Address, String};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, BytesN, Env, IntoVal, Vec,
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

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
    assert_eq!(ct.manager(), manager);
    assert_eq!(ct.components().len(), 3);
}

#[test]
fn mint() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin1 = Address::random(&e);
    let mut admin2 = Address::random(&e);

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    let user1 = Address::random(&e);
    token1.mint(&user1, &5000);
    let components = vec![
        &e,
        token1.address.clone(),
        // token2.address.clone()
    ];

    assert_eq!(token1.balance(&user1), 5000);

    let amounts = vec![&e, 100]; //, 1000];
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
    token1.approve(&user1, &ct.address, &1000i128, &200);
    ct.mint(&user1, &2); // mints 2 ctokens / requires 200 of the componnet
    assert_eq!(ct.balance(&user1), 2);
    assert_eq!(token1.balance(&ct.address), 200);
}

#[test]
fn burn() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin1 = Address::random(&e);
    let mut admin2 = Address::random(&e);

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    let user1 = Address::random(&e);
    token1.mint(&user1, &1000);
    token2.mint(&user1, &2000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    assert_eq!(token1.balance(&user1), 1000);

    let amounts = vec![&e, 100, 200]; //, 1000];
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
    token1.approve(&user1, &ct.address, &1000i128, &200);
    token2.approve(&user1, &ct.address, &2000i128, &200);
    ct.mint(&user1, &10);  
    assert_eq!(ct.balance(&user1), 10);
    assert_eq!(token1.balance(&ct.address), 1000);
    assert_eq!(token2.balance(&ct.address), 2000);
    assert_eq!(token1.balance(&user1), 0);
    assert_eq!(token2.balance(&user1), 0);

    ct.burn(&user1, &1);
    assert_eq!(ct.balance(&user1), 9);
    assert_eq!(token1.balance(&ct.address), 900);
    assert_eq!(token2.balance(&ct.address), 1800);

    assert_eq!(token1.balance(&user1), 100);
    assert_eq!(token2.balance(&user1), 200);

    ct.burn(&user1, &4);
    assert_eq!(ct.balance(&user1), 5);
    assert_eq!(token1.balance(&ct.address), 500);
    assert_eq!(token2.balance(&ct.address), 1000);
    assert_eq!(token1.balance(&user1), 500);
    assert_eq!(token2.balance(&user1), 1000);

    ct.burn(&user1, &5);
    assert_eq!(ct.balance(&user1), 0);
    assert_eq!(token1.balance(&ct.address), 0);
    assert_eq!(token1.balance(&user1), 1000);
}
