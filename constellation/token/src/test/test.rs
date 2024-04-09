
extern crate std;

use crate::contract::ConstellationTokenClient;
use std::println;

use crate::error::Error;

use super::soroswap::add_liquidity::add_liquidity_v2;
use super::soroswap::{self, SoroswapRouterTest};
// use super::test_interface::initialize_token;
use constellation_lib::traits::adapter::dex::Interface;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, BytesN, Env, IntoVal, Symbol, Val, Vec,
};
use soroban_sdk::{Address, String};

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
}

pub mod adapter {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_adapter_soroswap.wasm"
    );
}
fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_constellation_token<'a>(e: &Env) -> ConstellationTokenClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::ConstellationToken {});
    let ct: ConstellationTokenClient<'_> = ConstellationTokenClient::new(e, contract_id);
    ct
}

fn create_adapter<'a>(e: &Env, router: &Address, factory: &Address) -> adapter::Client<'a> {
    adapter::Client::new(e, &e.register_contract_wasm(None, adapter::WASM))
}

// #[test]
// fn swap_tokens_for_exact_tokens_amount_in_should() {
//     let test = SoroswapRouterTest::setup();
//     test.env.budget().reset_unlimited();
//     test.contract.initialize(&test.factory.address);
//     let deadline: u64 = test.env.ledger().timestamp() + 1000;

//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());

//     let amount_0: i128 = 1_000_000_000;
//     let amount_1: i128 = 4_000_000_000;

//     // -->
//     let mut token_2 = soroswap::create_token_contract(&test.env, &test.admin);
//     let mut token_3 = soroswap::create_token_contract(&test.env, &test.admin);

//     if &token_3.address < &token_2.address {
//         std::mem::swap(&mut token_2, &mut token_3);
//     }
//     token_2.mint(&test.user, &10_000_000_000_000_000_000);
//     token_3.mint(&test.user, &10_000_000_000_000_000_000);

//     add_liquidity_v2(
//         &test,
//         &test.token_0.address,
//         &test.token_1.address,
//         &amount_0,
//         &amount_1,
//     );
//     add_liquidity_v2(
//         &test,
//         &token_2.address,
//         &token_3.address,
//         &amount_0,
//         &amount_1,
//     );

//     add_liquidity_v2(
//         &test,
//         &test.token_0.address,
//         &token_2.address,
//         &amount_0,
//         &amount_1,
//     );
//     add_liquidity_v2(
//         &test,
//         &test.token_0.address,
//         &token_3.address,
//         &amount_0,
//         &amount_1,
//     );

//     add_liquidity_v2(
//         &test,
//         &test.token_1.address,
//         &token_2.address,
//         &amount_0,
//         &amount_1,
//     );
//     add_liquidity_v2(
//         &test,
//         &test.token_1.address,
//         &token_3.address,
//         &amount_0,
//         &amount_1,
//     );
//     // -- //

//     let expected_amount_out = 5_000_000;
//     let amount_in_should = test
//         .contract
//         .router_get_amounts_in(&expected_amount_out, &path)
//         .get(0)
//         .unwrap();

//     let amounts = test.contract.swap_tokens_for_exact_tokens(
//         &expected_amount_out, //amount_out
//         &(amount_in_should),  // amount_in_max
//         &path,                // path
//         &test.user,           // to
//         &deadline,
//     ); // deadline

//     &test.env.mock_all_auths();
//     let components = vec![
//         &test.env,
//         test.token_0.address.clone(),
//         test.token_1.address.clone(),
//     ];
//     let amounts = vec![&test.env, 1000, 1000];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&test.env);
//     let symbol = "token_symbol".into_val(&test.env);
//     let admin = Address::generate(&test.env);
//     let manager = Address::generate(&test.env);
//     let ct: ConstellationTokenClient<'_> = create_constellation_token(&test.env);

//     ct.initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     let allowance = 10_200_000i128;

//     let mut args: Vec<Val> = vec![&test.env];
//     args.push_back(ct.address.into_val(&test.env));
//     args.push_back(test.contract.address.into_val(&test.env));
//     args.push_back(allowance.into_val(&test.env));
//     args.push_back(10000u32.into_val(&test.env));

//     ct.invoke(
//         &token_2.address,
//         &test.token_0.address,
//         &(Symbol::new(&test.env, "approve"), args),
//         &vec![&test.env],
//     );

//     assert_eq!(
//         test.token_0.allowance(&ct.address, &test.contract.address),
//         allowance
//     );

//     test.token_0
//         .approve(&test.user, &ct.address, &1_200_000i128, &1000u32);
//     test.token_1
//         .approve(&test.user, &ct.address, &1_200_000i128, &1000u32);
//     ct.mint(&test.user, &10i128);

//     // trade
//     let path: Vec<Address> = vec![
//         &test.env,
//         test.token_0.address.clone(),
//         token_2.address.clone(),
//     ];
//     let amount_in = 1000; //1_000_000i128;
//     let res = &test.contract.router_get_amounts_out(&amount_in, &path);
//     let amount_out = res.get(1).unwrap();

//     let mut args: Vec<Val> = vec![&test.env];
//     args.push_back(amount_in.into_val(&test.env));
//     args.push_back(amount_out.into_val(&test.env));
//     args.push_back(path.into_val(&test.env));
//     args.push_back(ct.address.into_val(&test.env));
//     args.push_back(deadline.into_val(&test.env));

//     let adapter = create_adapter(&test.env, &test.contract.address, &test.factory.address);
//     adapter.initialize(&test.contract.address, &test.factory.address);
//     let auth_entries = adapter.create_sub_auth(
//         &amount_in,
//         &test.token_0.address.clone(),
//         &token_2.address,
//         &ct.address,
//     );
//     let bal = token_2.balance(&ct.address);
//     assert_eq!(bal, 0);
//     ct.invoke(
//         &token_2.address,
//         &test.contract.address,
//         &(Symbol::new(&test.env, "swap_exact_tokens_for_tokens"), args),
//         &auth_entries,
//     );
//     let bal = token_2.balance(&ct.address);
//     assert_eq!(bal, amount_out);
// }

// #[test]
// fn test_initialize_should_panic_with_already_initalized() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let components = vec![
//         &e,
//         Address::generate(&e),
//         Address::generate(&e),
//         Address::generate(&e),
//     ];
//     let amounts = vec![&e, 100, 100, 100];
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

//     let res = ct.try_initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     assert_eq!(res, Err(Ok(Error::AlreadyInitalized)));
//     // assert_eq!(ct.get_admin(), admin);
//     assert_eq!(ct.get_manager().unwrap(), manager);
//     assert_eq!(ct.get_components().len(), 3);
// }

// #[test]
// fn test_initialize_should_panic_with_components_amounts_length_mismatch() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let components = vec![
//         &e,
//         Address::generate(&e),
//         Address::generate(&e),
//         Address::generate(&e),
//         Address::generate(&e),
//     ];
//     let amounts = vec![&e, 100, 100, 100];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

//     let res = ct.try_initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     assert_eq!(res, Err(Ok(Error::ComponentsAmountsLengthMismatch)));
// }

// #[test]
// fn test_initialize_should_panic_with_zero_components() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let components = vec![&e];
//     let amounts: Vec<i128> = vec![&e];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

//     let res = ct.try_initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     assert_eq!(res, Err(Ok(Error::ZeroComponents)));
// }

// #[test]
// fn test_initialize_should_panic_with_zero_or_negative_amount_not_allowed_1() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let components = vec![
//         &e,
//         Address::generate(&e),
//         Address::generate(&e),
//         Address::generate(&e),
//     ];
//     let amounts = vec![&e, 100, 100, -1];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

//     let res = ct.try_initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     assert_eq!(res, Err(Ok(Error::ZeroOrNegativeAmount)));
// }

// #[test]
// fn test_initialize_should_panic_with_zero_or_negative_amount_not_allowed_2() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let components = vec![
//         &e,
//         Address::generate(&e),
//         Address::generate(&e),
//         Address::generate(&e),
//     ];
//     let amounts = vec![&e, 100, 100, 0];
//     let decimal: u32 = 6;
//     let name = "c_token".into_val(&e);
//     let symbol = "token_symbol".into_val(&e);
//     let admin = Address::generate(&e);
//     let manager = Address::generate(&e);
//     let ct: ConstellationTokenClient<'_> = create_constellation_token(&e);

//     let res = ct.try_initialize(
//         &decimal,
//         &components,
//         &amounts,
//         &name,
//         &symbol,
//         &admin,
//         &manager,
//     );

//     assert_eq!(res, Err(Ok(Error::ZeroOrNegativeAmount)));
// }

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
    let name: String = "c_token".into_val(&e);
    let symbol: String = "token_symbol".into_val(&e);
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

   assert_eq!(ct.get_admin().unwrap(), admin);
   assert_eq!(ct.get_manager().unwrap(), manager);
   assert_eq!(ct.get_components().len(), 3);
}

// #[test]
// fn test_set_manager_panics_with_authorization_failed() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let new_manager = Address::generate(&e);
//     let token1 = create_token_contract(&e, &Address::generate(&e));
//     let token2 = create_token_contract(&e, &Address::generate(&e));
//     let token3 = create_token_contract(&e, &Address::generate(&e));

//     let (ct, admin, manager) = initialize_token(
//         &e,
//         create_constellation_token(&e),
//         (token1.address, token2.address, token3.address),
//     );
//     ct.set_manager(&new_manager);
//     assert_eq!(
//         e.auths(),
//         std::vec![(
//             manager.clone(),
//             AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     ct.address.clone(),
//                     "set_manager".into_val(&e),
//                     (&new_manager,).into_val(&e),
//                 )),
//                 sub_invocations: std::vec![]
//             }
//         )]
//     );
//     assert_eq!(ct.get_manager().unwrap(), new_manager);
// }

// #[test]
// fn mint_reverts_with_zero_or_negative_amount_not_allowed() {
//     let e = Env::default();
//     e.mock_all_auths();
//     let mint_to = Address::generate(&e);
//     let new_manager = Address::generate(&e);

//     let token1 = create_token_contract(&e, &Address::generate(&e));
//     let token2 = create_token_contract(&e, &Address::generate(&e));
//     let token3 = create_token_contract(&e, &Address::generate(&e));

//     let (ct, _, _) = initialize_token(
//         &e,
//         create_constellation_token(&e),
//         (token1.address, token2.address, token3.address),
//     );

//     let restult = ct.try_mint(&mint_to, &i128::from(0));
//     assert_eq!(restult, Err(Ok(Error::ZeroOrNegativeAmount.into())));
// }
