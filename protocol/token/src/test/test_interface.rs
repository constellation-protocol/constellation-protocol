#![cfg(test)]
extern crate std;

use crate::error::Error;
use crate::ConstellationTokenClient; //crate::{contract::Token, ConstellationTokenClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, Env, IntoVal, Symbol,
};
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

pub(super) fn initialize_token<'a>(
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
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let mut token = create_constellation_token(&e);

    let (mut token, admin1, manager) = initialize_token(&e, token);

    let admin2 = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);

    token.mint(&user1, &1000);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user2, &user3, &500, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 500);

    token.transfer(&user1, &user2, &600);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    token.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        e.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);

    token.transfer(&user1, &user3, &300);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user3), 300);

    token.set_admin(&admin2);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // Increase to 500
    token.approve(&user2, &user3, &500, &200);
    assert_eq!(token.allowance(&user2, &user3), 500);
    token.approve(&user2, &user3, &0, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 0);
}

#[test]
fn test_burn_from_panics_with_zero_or_negative_amount_not_allowed() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user2, &500, &200);
    assert_eq!(token.allowance(&user1, &user2), 500);

    let result = token.try_burn_from(&user2, &user1, &0);
    assert_eq!(result, Err(Ok(Error::ZeroOrNegativeAmountNotAllowed.into())));
}

#[test]
fn test_burn_from_panics_with_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user2, &500, &200);
    assert_eq!(token.allowance(&user1, &user2), 500);

    let result = token.try_burn_from(&user2, &user1, &600);
    assert_eq!(result, Err(Ok(Error::InsufficientAllowance.into())));
}


#[test]
fn test_burn_panics_with_zero_or_negative_amount_not_allowed() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);

    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    let result = token.try_burn(&user1, &0);
    assert_eq!(result, Err(Ok(Error::ZeroOrNegativeAmountNotAllowed.into())));
}

#[test]
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user2, &500, &200);
    assert_eq!(token.allowance(&user1, &user2), 500);

    token.burn_from(&user2, &user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.allowance(&user1, &user2), 0);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user2), 0);

    token.burn(&user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
}

#[test]
fn transfer_panics_with_zero_or_negative_amount_not_allowed() {
    let e = Env::default();
    e.mock_all_auths();

    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    let result = token.try_transfer(&user1, &user2, &0);
    assert_eq!(result, Err(Ok(Error::ZeroOrNegativeAmountNotAllowed.into())));
}

#[test]
fn transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    let result = token.try_transfer(&user1, &user2, &1001);
    assert_eq!(result, Err(Ok(Error::InsufficientBalance.into())));
}

#[test]
fn transfer_from_with_zero_or_negative_amount_not_allowed() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    
    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);


    let result = token.try_transfer_from(&user3, &user1, &user2, &0);
    assert_eq!(result, Err(Ok(Error::ZeroOrNegativeAmountNotAllowed.into())));
}


#[test]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    
    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);
    let result = token.try_transfer_from(&user3, &user1, &user2, &101);
    assert_eq!(result, Err(Ok(Error::InsufficientAllowance.into())));
}

#[test]
fn transfer_from_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    
    let mut token = create_constellation_token(&e);
    let (mut token, admin1, manager) = initialize_token(&e, token);

     token.mint(&user1, &1000);
     assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &1001, &200);
    assert_eq!(token.allowance(&user1, &user3), 1001);
    let result = token.try_transfer_from(&user3, &user1, &user2, &1001);
    assert_eq!(result, Err(Ok(Error::InsufficientBalance.into())));
}

#[test]
// #[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let e = Env::default();
    let admin = Address::generate(&e);

    let mut token = create_constellation_token(&e);
  
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

    let result = token.try_initialize(
        &(u32::from(u8::MAX) + 1),
        &components,
        &amounts,
        &name,
        &symbol,
        &admin,
        &manager,
    );
    assert_eq!(result, Err(Ok(Error::ValueTooLargeOverFlow.into())));

}

#[test]
fn test_zero_allowance() {
    // Here we test that transfer_from with a 0 amount does not create an empty allowance
    let e = Env::default();
    e.mock_all_auths();

    let spender = Address::generate(&e);
    let from = Address::generate(&e);
    let mut token = create_constellation_token(&e);

    assert!(token.get_allowance(&from, &spender).is_none());
}
 