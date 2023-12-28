#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, BytesN, Env, InvokeError, Val,
};
// use soroban_env_common
use crate::{
    contract::{constellation_token, MinterBurner, MinterBurnerClient},
    error::Error,
};
use soroban_sdk::IntoVal;

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
fn create_minter_burner<'a>(e: &Env) -> MinterBurnerClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::MinterBurner {});
    let ct: MinterBurnerClient<'_> = MinterBurnerClient::new(e, contract_id);
    ct
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
    let minter_burner = create_minter_burner(&e);
    let result = minter_burner.try_mint(&user, &ct.address, &0i128);

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
    token1.mint(&user1, &5000);
    token2.mint(&user1, &2000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 1000, 2000];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let minter_burner = create_minter_burner(&e);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &minter_burner.address,
        &manager,
    );

    token1.approve(&user1, &minter_burner.address, &10000i128, &1000);
    token2.approve(&user1, &minter_burner.address, &10000i128, &1000);
    let res = minter_burner.try_mint(&user1, &ct.address, &100); // mints 2 ctokens / requires 200 of the componnet
    assert_eq!(
        res,
        Err(Err(InvokeError::Contract(
            9 /*BalanceError - stellat asset contract errro code*/
        )
        .into()))
    );
    assert_eq!(ct.balance(&user1), 0);
    assert_eq!(token1.balance(&user1), 5000);
    assert_eq!(token2.balance(&user1), 2000);
}

#[test]
fn mint_should_fail_with_token_contract_insufficient_balance_and_revert() {
    let e = Env::default();
    e.mock_all_auths();
    let mut admin = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin);
    let token2 = create_token_contract(&e, &admin);

    let user1 = Address::generate(&e);
    token1.mint(&user1, &5000);
    token2.mint(&user1, &2000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let amounts = vec![&e, 1000, 2000];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);
    let minter_burner = create_minter_burner(&e);
    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &minter_burner.address,
        &manager,
    );

    token1.approve(&user1, &minter_burner.address, &10000i128, &1000);
    token2.approve(&user1, &minter_burner.address, &10000i128, &1000);
    let res = minter_burner.try_mint(&user1, &ct.address, &2); // mints 2 ctokens / requires 200 of the componnet
    assert_eq!(
        res,
        Err(Err(InvokeError::Contract(
            10 /*BalanceError - stellat asset contract errro code*/
        )
        .into()))
    );
    assert_eq!(ct.balance(&user1), 0);
    assert_eq!(token1.balance(&user1), 5000);
    assert_eq!(token2.balance(&user1), 2000);
}

// #[test]
// // #[should_panic(expected = "insufficient balance")]
// fn mint_should_fail_with_insufficient_balance_and_revert() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let mut admin1 = Address::generate(&e);
//     let mut admin2 = Address::generate(&e);

//     let token1 = create_token_contract(&e, &admin1);
//     let token2 = create_token_contract(&e, &admin2);

//     let user1 = Address::generate(&e);
//     token1.mint(&user1, &5000);
//     token2.mint(&user1, &5000);
//     let components = vec![&e, token1.address.clone(), token2.address.clone()];

//     let amounts = vec![&e, 1000, 2000];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let (ct, ct_id) = create_constellation_token(&e);
//     let minter_burner = create_minter_burner(&e);
//     ct.initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &minter_burner.address,
//         &manager,
//     );

//     token1.approve(&user1, &minter_burner.address, &5000i128, &1000);
//     token2.approve(&user1, &minter_burner.address, &5000i128, &1000);
//     let res = ct.try_mint(&user1, &3); // mints 2 ctokens / requires 200 of the componnet
//                                        // assert_eq!(res, Err(Ok(Error::InsufficientBalance)));
//     assert_eq!(token1.balance(&user1), 5000);
//     assert_eq!(token2.balance(&user1), 5000);
// }

// #[test]
// fn mint() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let mut admin1 = Address::generate(&e);
//     let mut admin2 = Address::generate(&e);

//     let token1 = create_token_contract(&e, &admin1);
//     let token2 = create_token_contract(&e, &admin2);

//     let user1 = Address::generate(&e);
//     token1.mint(&user1, &5000);
//     let components = vec![
//         &e,
//         token1.address.clone(),
//         // token2.address.clone()
//     ];

//     assert_eq!(token1.balance(&user1), 5000);

//     let amounts = vec![&e, 100]; //, 1000];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let (ct, ct_id) = create_constellation_token(&e);
//     let minter_burner = create_minter_burner(&e);

//     ct.initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &minter_burner.address,
//         &manager,
//     );

//     token1.approve(&user1, &minter_burner.address, &1000i128, &200);
//     minter_burner.mint(&user1, &ct.address, &2); // mints 2 ctokens / requires 200 of the componnet
//     assert_eq!(ct.balance(&user1), 2);
//     assert_eq!(token1.balance(&ct.address), 200);
// }

// #[test]
// fn burn() {
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
//     assert_eq!(ct.balance(&user1), 10);
//     assert_eq!(token1.balance(&ct.address), 1000);
//     assert_eq!(token2.balance(&ct.address), 2000);
//     assert_eq!(token1.balance(&user1), 0);
//     assert_eq!(token2.balance(&user1), 0);

//     ct.burn(&user1, &1);
//     assert_eq!(
//         e.auths(),
//         std::vec![(
//             admin.clone(),
//             AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     ct.address.clone(),
//                     symbol_short!("burn"),
//                     (&user1, 1_i128,).into_val(&e),
//                 )),
//                 sub_invocations: std::vec![]
//             }
//         )]
//     );

//     assert_eq!(ct.balance(&user1), 9);
//     assert_eq!(token1.balance(&ct.address), 900);
//     assert_eq!(token2.balance(&ct.address), 1800);

//     assert_eq!(token1.balance(&user1), 100);
//     assert_eq!(token2.balance(&user1), 200);

//     ct.burn(&user1, &4);
//     assert_eq!(ct.balance(&user1), 5);
//     assert_eq!(token1.balance(&ct.address), 500);
//     assert_eq!(token2.balance(&ct.address), 1000);
//     assert_eq!(token1.balance(&user1), 500);
//     assert_eq!(token2.balance(&user1), 1000);

//     ct.burn(&user1, &5);
//     assert_eq!(ct.balance(&user1), 0);
//     assert_eq!(token1.balance(&ct.address), 0);
//     assert_eq!(token1.balance(&user1), 1000);
//     assert_eq!(token2.balance(&ct.address), 0);
//     assert_eq!(token2.balance(&user1), 2000);
// }

// #[test]
// #[should_panic(expected = "insufficient balance")]
// fn test_burn_should_panic_with_insufficient_balance() {
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
