use core::ops::Add;

use super::clients::{
    create_constellation_token, create_factory, create_router, create_soroswap_router,
    create_token_contract, ConstellationTokenClient,
};
use crate::factory;
use crate::token::constellation_token;
use crate::{
    contract::{Router, RouterClient},
    error::Error,
};

use super::setup::TradeTest;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, BytesN, Env, InvokeError, String, Val, Vec,
};
use soroban_sdk::{xdr, IntoVal};
extern crate std;
use crate::auth::*;
use crate::helper::*;
use crate::token::Component;

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

    let (ct, _, _) = initialize_token(&e, create_constellation_token(&e));
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
    let router = create_router(&e);
    let factory = create_factory(&e);
    let result = router.try_create_token(
        &decimal,
        &name,
        &symbol,
        &manager,
        &components,
        &amounts,
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
    let wasm_hash = e.deployer().upload_contract_wasm(constellation_token::WASM);
    let router = create_router(&e);
    let factory = create_factory(&e);
    factory.initialize(&user1, &wasm_hash);
    let soroswap_router = create_soroswap_router(&e);
    router.initialize(
        &factory.address,
        &soroswap_router.address,
    );

    let result = router.create_token(
        &decimal,
        &name,
        &symbol,
        &manager,
        &components,
        &amounts,
    );
    let tokens = factory.get_token_list();
    assert_eq!(result, tokens.get(0).unwrap());
} 
#[test]
fn test_mint() {
    let test = TradeTest::setup();
    // test.env.mock_all_auths_allowing_non_root_auth();

    // units
    let units = vec![&test.env, 1, 1];
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

    test.tokens.0.approve(
        &test.user,
        &test.router.address,
        &10000_000_000i128,
        &1000u32,
    );

    test.tokens.0.approve(
        &test.router.address,
        &test.s_router.address,
        &700000_000_000i128,
        &1000u32,
    );

    let amount_in = 1000;

    let path = &vec![
        &test.env,
        test.tokens.0.address.clone(),
        test.tokens.1.address.clone(),
    ];

    let res = test.s_router.router_get_amounts_out(&amount_in, path);

    let amount_out = res.get(1).unwrap();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let pair = test
        .s_router
        .router_pair_for(&test.tokens.1.address, &test.tokens.0.address);

    assert_eq!(test.constellation_token.balance(&test.user), 0);

    let mint_amount = 1;

    let refund = test.router.mint_exact_constellation(
        &mint_amount,
        &amount_in,
        &test.tokens.0.address,
        &test.user,
        &test.constellation_token.address,
        &deadline,
    );

    assert_eq!(test.constellation_token.balance(&test.user), 1);
    // assert_eq!(refund, 0);

    std::dbg!(test.env.auths());
}

#[test]
fn test_redeem_to() {
    let test = TradeTest::setup();
    let test = TradeTest::setup();
    let redeem_address = Address::generate(&test.env);
    // test.env.mock_all_auths_allowing_non_root_auth();

    // units
    let units = vec![&test.env, 1, 1];
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
        &7u32,
        &components,
        &units,
        &name,
        &symbol,
        &test.router.address,
        &manager,
    );

    let approve_amount = 10_000_000 * 10i128.pow(7);

    test.tokens.1.approve(
        &test.user,
        &test.constellation_token.address,
        &approve_amount,
        &1000u32,
    );

    test.tokens.2.approve(
        &test.user,
        &test.constellation_token.address,
        &approve_amount,
        &1000u32,
    );

    test.tokens
        .0
        .approve(&test.user, &test.router.address, &approve_amount, &1000u32);

    test.tokens.0.approve(
        &test.router.address,
        &test.s_router.address,
        &approve_amount,
        &1000u32,
    );

    let mint_amount = 1 * 10i128.pow(7);
    //
    let mut comp = vec![&test.env];
    for c in test.constellation_token.get_components().iter() {
        comp.push_back(Component {
            address: c.address.clone(),
            unit: c.unit.clone(),
        })
    }

    let (amount_in, _) =
        test.router
            .get_required_amount_token_in(&test.tokens.0.address, &mint_amount, &comp);

    // let amount_in = 1000 * 10i128.pow(7)
    let path = &vec![
        &test.env,
        test.tokens.0.address.clone(),
        test.tokens.1.address.clone(),
    ];

    let res = test.s_router.router_get_amounts_out(&amount_in, path);

    let amount_out = res.get(1).unwrap();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let pair = test
        .s_router
        .router_pair_for(&test.tokens.1.address, &test.tokens.0.address);

    assert_eq!(test.constellation_token.balance(&test.user), 0);

    let refund = test.router.mint_exact_constellation(
        &mint_amount,
        &amount_in,
        &test.tokens.0.address,
        &test.user,
        &test.constellation_token.address,
        &deadline,
    );
    test.constellation_token
        .approve(&test.user, &test.router.address, &mint_amount, &200);

    let initial_balance = test.tokens.0.balance(&test.user);
    test.router.redeem_into(
        &test.user,
        &mint_amount,
        &test.constellation_token.address,
        &test.tokens.0.address,
        &test.deadline,
    );
    let final_balance = test.tokens.0.balance(&test.user);
}
