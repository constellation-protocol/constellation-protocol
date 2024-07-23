use super::clients::{
    ConstellationTokenClient,
    create_constellation_token, create_factory, create_router, create_soroswap_router,
    create_token_contract,
};
use crate::factory;
use crate::token::constellation_token;
use crate::{
    contract::{Router, RouterClient},
    error::Error,
};

use super::setup::TradeTest;
use soroban_sdk::IntoVal;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, BytesN, Env, InvokeError, String, Val, Vec,
};

pub(crate) fn initialize_token<'a>(
    e: &Env,
    ct: ConstellationTokenClient<'a>,
) -> (ConstellationTokenClient<'a>, Address, Address) {
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

    let (ct,_,_)= initialize_token(&e, create_constellation_token(&e));
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
    let ct = create_constellation_token(&e);
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
    let ct = create_constellation_token(&e);
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
    let ct = create_constellation_token(&e);
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
    router.mint(&user1, &ct.address, &2);
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
    let ct = create_constellation_token(&e);
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
    let ct = create_constellation_token(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);
    let router = create_router(&e);
    let factory = create_factory(&e);
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
    let s = TradeTest::setup();
    let e = Env::default();
    e.mock_all_auths();
    let mut admin = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin);
    let token2 = create_token_contract(&e, &admin);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &50000000000);
    token2.mint(&user1, &20000000000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts: Vec<i128> = vec![&e, 10000000000, 20000000000];
    let decimal: u32 = 7;
    let name: String = "c_token".into_val(&e);
    let symbol: String = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    // let ct_client= create_constellation_token(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);
    let router = create_router(&e);
    let factory = create_factory(&e);
    let soroswap_router = create_soroswap_router(&e);
    router.initialize(
        &factory.address,
        &soroswap_router.address,
        &Address::generate(&e),
    );

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

#[test]
fn test_mint() {
    let test = TradeTest::setup();
    test.env.mock_all_auths();
    // units
    let units = vec![&test.env, 1000, 1000];
    // components
    let components: Vec<Address> = vec![
        &test.env,
        test.tokens.1.address.clone(),
        test.tokens.2.address.clone(),
    ];
    let name: String = "c_token".into_val(&test.env);
    let symbol: String = "token_symbol".into_val(&test.env);
    let manager = Address::generate(&test.env);

    test.constellation_token.initialize(
        &6u32,
        &components,
        &units,
        &name,
        &symbol,
        &test.router.address,
        &manager,
    );

    test.tokens.1.approve(
        &test.user,
        &test.constellation_token.address,
        &10_000_000i128,
        &1000u32,
    );

    test.tokens.2.approve(
        &test.user,
        &test.constellation_token.address,
        &10_000_000i128,
        &1000u32,
    );

    test.tokens
        .0
        .approve(&test.user, &test.router.address, &10_000_000i128, &1000u32);

    let refund = test.router.mint_exact_constellation(
        &1_000_000i128,
        &10i128,
        &test.tokens.0.address,
        &test.constellation_token.address,
        &test.user,
        &10000000u64,
    );

    // assert_eq!(refund, 10);

    assert_eq!(test.constellation_token.balance(&test.user), 10);
}

#[test]
fn test_redeem_to() {
    let test = TradeTest::setup();
    test.env.mock_all_auths();
    // units
    let units = vec![&test.env, 1000, 1000];
    // components
    let components: Vec<Address> = vec![
        &test.env,
        test.tokens.1.address.clone(),
        test.tokens.2.address.clone(),
    ];
    let name: String = "c_token".into_val(&test.env);
    let symbol: String = "token_symbol".into_val(&test.env);
    let manager = Address::generate(&test.env);

    test.constellation_token.initialize(
        &6u32,
        &components,
        &units,
        &name,
        &symbol,
        &test.router.address,
        &manager,
    );

    test.tokens.1.approve(
        &test.user,
        &test.constellation_token.address,
        &10_000_000i128,
        &1000u32,
    );

    test.tokens.2.approve(
        &test.user,
        &test.constellation_token.address,
        &10_000_000i128,
        &1000u32,
    );

    test.tokens
        .0
        .approve(&test.user, &test.router.address, &10_000_000i128, &1000u32);

    let b1 = test.tokens.0.balance(&test.user);
    // assert_eq!(b1, 9999999988000000000);
    let ct_amount = &300i128;
    let refund = test.router.mint_exact_constellation(
        &1_000_000i128,
        ct_amount,
        &test.tokens.0.address,
        &test.constellation_token.address,
        &test.user,
        &test.deadline,
    );
    let b1 = test.tokens.0.balance(&test.user);
    assert_eq!(b1, 9999999987998792572); // removed - 40960

    test.constellation_token
        .approve(&test.user, &test.router.address, ct_amount, &200);
    test.router.redeem_into(
        &test.user,
        ct_amount,
        &test.constellation_token.address,
        &test.tokens.0.address,
        &test.deadline,
    );

    let cb =  test.constellation_token.balance(&test.user);
    assert_eq!(cb, 0);
    assert_eq!(test.tokens.0.balance(&test.user), 9999999987999390816);

}
