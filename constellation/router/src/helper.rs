use crate::error::Error;
use crate::require::require_exchange_router;
use crate::soroswap_router;
use crate::soroswap_router::router_get_amounts_in;
use crate::soroswap_router::router_pair_for;
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
                let (function, args) = get_swap_call_data(
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

pub fn get_base_token_amount_in(
    e: &Env,
    router_id: &Address,
    amount_in: i128,
    amount_out_min: i128,
    token_in: &Address,
    token_out: &Address,
    to: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    let amount_in = match token_in != token_out {
        true => soroswap_router::swap_exact_tokens_for_tokens(
            &e,
            router_id,
            amount_in,
            amount_out_min,
            token_in,
            token_out,
            &to.clone(),
            deadline,
        )?,
        false => amount_in,
    };
    Ok(amount_in)
}

pub fn refund_unspent(
    e: &Env,
    refund: i128,
    token_in: &Address,
    to: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    token::Client::new(&e, &token_in).transfer(&e.current_contract_address(), &to, &refund);
    Ok(refund)
}
