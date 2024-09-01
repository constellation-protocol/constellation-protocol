use crate::error::Error;
use crate::require::require_exchange_router;
use crate::soroswap_router;
use crate::soroswap_router::{router_get_amounts_in, router_get_amounts_out, router_pair_for};
use crate::token::Component;
use crate::{auth::*, token::invoke};
use soroban_sdk::xdr;
use soroban_sdk::{token, vec, Address, Env, Val, Vec};

extern crate std;

pub fn get_required_amount_token_in(
    e: &Env,
    token_in_id: &Address,
    amount_constellation: i128,
    components: &Vec<Component>,
) -> Result<(i128, Vec<i128>), Error> {
    let router_id = require_exchange_router(&e);
    let mut total_token_in_amount = 0;
    let mut token_amounts_in: Vec<i128> = vec![e]; // amounts of each component token required

    for Component { unit, address } in components.iter() {
        let component_in_amount = amount_constellation * unit;
        let path = vec![
            e,
            token_in_id.clone(),
            address.clone(), /* component token address */
        ];

        let amounts_in = router_get_amounts_in(e, component_in_amount, &router_id, &path);

        match amounts_in.get(0) {
            Some(amount) => {
                total_token_in_amount += amount;
                token_amounts_in.push_back(amount);
            }
            None => {
                return Err(Error::AmountsInError);
            }
        }
    }

    Ok((total_token_in_amount, token_amounts_in))
}

pub fn swap_exact_tokens_for_tokens(
    e: &Env,
    router_id: &Address,
    token_out: &Address,
    components: &Vec<Component>,
    to: &Address,
    deadline: u64,
) {
    for c in components.iter() {
        let token_client = token::Client::new(&e, &c.address);
        let amount_in = token_client.balance(&e.current_contract_address());
        token_client.approve(&e.current_contract_address(), router_id, &amount_in, &1000);
        let pair = router_pair_for(e, &router_id, &c.address.clone(), &token_out.clone());

        let results = router_get_amounts_out(
            e,
            amount_in,
            router_id,
            &vec![e, c.address.clone(), token_out.clone()],
        );

        let amount_out = results.get(1).unwrap();

        let (function, args) = get_swap_exact_tokens_for_tokens_call_data(
            e,
            c.address.clone(),
            token_out.clone(),
            amount_in,
            amount_out,
            to.clone(),
            deadline,
        );

        let auth_entries = create_sub_auth(
            e,
            amount_in,
            c.address.clone(),
            token_out.clone(),
            to.clone(),
            pair.clone(),
        );

        e.authorize_as_current_contract(auth_entries);
        e.invoke_contract(router_id, &function, args)
    }
}

pub fn swap_tokens_for_exact_tokens(
    e: &Env,
    mint_amount: &i128,
    token_in: &Address,
    to: &Address,
    router_id: &Address, // soroswap router
    token_amounts_in: &Vec<i128>,
    components: &Vec<Component>,
    constellation_token_id: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    let mut total_spent = 0;
    for (i, c) in components.iter().enumerate() {
        let pair = router_pair_for(e, &router_id, &token_in.clone(), &c.address.clone());
        let token_client = token::Client::new(&e, &c.address);

        let amount_out = c.unit * mint_amount;
        match token_amounts_in.get(i as u32) {
            Some(amount_in) => {
                let (function, args) = get_swap_tokens_for_exact_tokens_call_data(
                    e,
                    token_in.clone(),
                    c.address.clone(),
                    amount_in,
                    amount_out,
                    to.clone(),
                    deadline,
                );
                let auth_entries = create_sub_auth(
                    e,
                    amount_in,
                    token_in.clone(),
                    c.address.clone(),
                    to.clone(),
                    pair.clone(),
                );

                e.authorize_as_current_contract(auth_entries);

                let result: Vec<i128> = e.invoke_contract::<Vec<i128>>(router_id, &function, args);

                let amount_in_spent: i128 = match result.get(0) {
                    Some(value) => value,
                    None => return Err(Error::SwapError),
                };

                // approve the constellation token to transfer the routers token
                token_client.approve(&to, &constellation_token_id, &amount_out, &1000u32);

                total_spent += amount_in_spent;
            }
            None => {
                return Err(Error::AmountsInError);
            }
        }
    }
    Ok(total_spent)
}

pub fn refund_unspent(e: &Env, refund: i128, token_in: &Address, to: &Address, deadline: u64) {
    token::Client::new(&e, &token_in).transfer(&e.current_contract_address(), &to, &refund);
}
