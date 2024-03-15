#![cfg(test)]

use crate::factory;
use crate::token::constellation_token;
use crate::{
    contract::{Router, RouterClient},
    error::Error,
};
use soroban_sdk::IntoVal;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, BytesN, Env, InvokeError, Val,
};

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}
fn create_constellation_token<'a>(e: &Env) -> (constellation_token::Client<'a>, Address) {
    let contract_id = &e.register_contract_wasm(None, constellation_token::WASM);
    let ct: constellation_token::Client<'_> = constellation_token::Client::new(e, contract_id);
    (ct, contract_id.clone())
}
fn create_router<'a>(e: &Env) -> RouterClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::Router {});
    let ct: RouterClient<'_> = RouterClient::new(e, contract_id);
    ct
}

fn create_factory<'a>(e: &Env) -> (factory::constellation_factory::Client<'a>, Address) {
    let contract_id = &e.register_contract_wasm(None, factory::constellation_factory::WASM);
    let factory: factory::constellation_factory::Client<'_> =
        factory::constellation_factory::Client::new(e, contract_id);
    (factory, contract_id.clone())
}

pub(crate) fn initialize_token<'a>(
    e: &Env,
    ct: constellation_token::Client<'a>,
) -> (constellation_token::Client<'a>, Address, Address) {
    let components = vec![
        &e,
        Address::generate(e),
        Address::generate(e),
        Address::generate(e),
    ];
    let amounts = vec![&e, 100, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(e);
    let symbol = "token_symbol".into_val(e);
    let admin = Address::generate(e);
    let manager = Address::generate(e);

    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );
    (ct, admin, manager)
}

#[test]
fn mint_test_should_fail_with_zero_or_negative_amount() {
    let e = Env::default();
    e.mock_all_auths();
    let mut user = Address::generate(&e);

    let (ct, _, _) = initialize_token(&e, create_constellation_token(&e).0);
    let router = create_router(&e);
    let result = router.try_mint(&user, &ct.address, &0i128);

    assert_eq!(result, Err(Ok(Error::ZeroOrNegativeAmount)));
}

#[test]
fn mint_should_fail_with_token_contract_insufficient_allowance_and_revert() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin);
    let token2 = create_token_contract(&e, &admin);
    let user1 = Address::generate(&e);
    token1.mint(&user1, &50000000000);
    token2.mint(&user1, &20000000000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 10000000000, 20000000000];
    let decimal: u32 = 7;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let router = create_router(&e);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &router.address,
        &manager,
    );

    token1.approve(&user1, &ct.address, &1000000000i128, &1000);
    token2.approve(&user1, &ct.address, &1000000000i128, &10000);
    let res = router.try_mint(&user1, &ct.address, &1000000000i128); // mints 2 ctokens / requires 200 of the componnet
    assert_eq!(
        res,
        Err(Err(InvokeError::Contract(
            9 /*AllowanceError - stellat asset contract error code*/
        )
        .into()))
    );
    assert_eq!(ct.balance(&user1), 0);
    assert_eq!(token1.balance(&user1), 50000000000);
    assert_eq!(token2.balance(&user1), 20000000000);
}

#[test]
fn mint_should_fail_with_insufficient_balance_and_revert() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin1 = Address::generate(&e);
    let mut admin2 = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &5000);
    token2.mint(&user1, &5000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 6000, 6000];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let router = create_router(&e);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &router.address,
        &manager,
    );

    token1.approve(&user1, &ct.address, &10000i128, &1000);
    token2.approve(&user1, &ct.address, &10000i128, &1000);
    let res = router.try_mint(&user1, &ct.address, &1);

    assert_eq!(
        res,
        Err(Err(InvokeError::Contract(
            10 /*BalanceError - stellat asset contract error code*/
        )
        .into()))
    );
    assert_eq!(token1.balance(&user1), 5000);
    assert_eq!(token2.balance(&user1), 5000);
}

#[test]
fn mint() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin1 = Address::generate(&e);
    let mut admin2 = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &5000);
    let components = vec![&e, token1.address.clone()];

    assert_eq!(token1.balance(&user1), 5000);

    let amounts = vec![&e, 100]; //, 1000];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let router = create_router(&e);

    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &router.address,
        &manager,
    );

    token1.approve(&user1, &ct.address, &1000i128, &200);
    router.mint(&user1, &ct.address, &2); // assert_eq!(token1.balance(&ct.address), 200);
}

#[test]
fn burn() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin1 = Address::generate(&e);
    let mut admin2 = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    token1.mint(&user1, &5000);
    let components = vec![&e, token1.address.clone()];

    assert_eq!(token1.balance(&user1), 5000);

    let amounts = vec![&e, 1000]; //, 1000];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let admin = Address::generate(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let router = create_router(&e);

    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &router.address,
        &manager,
    );

    token1.approve(&user1, &ct.address, &2000i128, &200);
    router.mint(&user1, &ct.address, &2); // mints 2 ctokens / requires 200 of the componnet
    assert_eq!(ct.balance(&user1), 2);
    ct.approve(&user1, &router.address, &2, &200);
    router.burn(&user1, &ct.address, &2);
}

// #[test]
// fn test_burn_should_fail_with_insufficient_balance() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let mut admin1 = Address::generate(&e);
//     let mut admin2 = Address::generate(&e);

//     let token1 = create_token_contract(&e, &admin1);
//     let token2 = create_token_contract(&e, &admin2);

//     let user1 = Address::generate(&e);
//     token1.mint(&user1, &1000);
//     token2.mint(&user1, &2000);
//     let components = vec![&e, token1.address.clone(), token2.address.clone()];
//     let amounts = vec![&e, 100, 200];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

//     ct.initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     token1.approve(&user1, &ct.address, &1000i128, &200);
//     token2.approve(&user1, &ct.address, &2000i128, &200);
//     ct.mint(&user1, &10);

//     ct.burn(&user1, &1001);
//     assert_eq!(ct.balance(&user1), 10);
// }

#[test]
fn create_token_fails_with_requires_factory() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin);
    let token2 = create_token_contract(&e, &admin);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &50000000000);
    token2.mint(&user1, &20000000000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 10000000000, 20000000000];
    let decimal: u32 = 7;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);
    let router = create_router(&e);
    let (factory, factory_address) = create_factory(&e);
    let result = router.try_create_token(
        &decimal,
        &name,
        &symbol,
        &manager,
        &components,
        &amounts,
        &wasm_hash,
        &wasm_hash,
    );

    assert_eq!(result, Err(Ok(Error::RequiresFactory)));
}

#[test]
fn create_token_succeeds() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin);
    let token2 = create_token_contract(&e, &admin);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &50000000000);
    token2.mint(&user1, &20000000000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 10000000000, 20000000000];
    let decimal: u32 = 7;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);
    let router = create_router(&e);
    let (factory, factory_address) = create_factory(&e);
    router.initialize(&factory_address);

    let result = router.create_token(
        &decimal,
        &name,
        &symbol,
        &manager,
        &components,
        &amounts,
        &wasm_hash,
        &wasm_hash,
    );
    let tokens = factory.get_token_list();
    assert_eq!(result, tokens.get(0).unwrap());
}