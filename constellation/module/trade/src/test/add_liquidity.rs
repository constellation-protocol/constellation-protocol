extern crate std;

use soroban_sdk::{
    testutils::{Address as _, Ledger, MockAuth, MockAuthInvoke},
    vec, Address, Env, IntoVal,
};

use super::clients::SoroswapRouterClient;

pub fn add_liquidity<'a>(
    env: &Env,
    router: &SoroswapRouterClient<'a>,
    user: &Address,
    token_0: &Address,
    token_1: &Address,
    amount_0: &i128,
    amount_1: &i128,
) -> (i128, i128, i128) {
    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    env.budget().reset_unlimited();
    router.add_liquidity(
        token_0,           //     token_a: Address,
        token_1,           //     token_b: Address,
        &amount_0,         //     amount_a_desired: i128,
        &amount_1,         //     amount_b_desired: i128,
        &0,                //     amount_a_min: i128,
        &0,                //     amount_b_min: i128,
        &user,             //     to: Address,
        &desired_deadline, //     deadline: u64,
    )
}
