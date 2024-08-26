use crate::error::Error;
use crate::soroswap_router::router_pair_for;
// use crate::storage::{factory, router};
use constellation_lib::traits::adapter::dex::IExchange;
use constellation_lib::traits::adapter::{self, dex};
use soroban_sdk::IntoVal;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error, vec, Address, BytesN, Env, String, Symbol, Val, Vec,
};

use crate::require::require_exchange_router;
static SWAP_TOKENS_FOR_EXACT_TOKENS: &'static str = "swap_tokens_for_exact_tokens";
static APPROVE: &'static str = "approve";
static TRANSFER: &'static str = "transfer";

// pub fn get_approve_call_data(
//         e: &Env,
//         from: Address,
//         spender: Address,
//         amount: i128,
//         expiration_ledger: u32,
//     ) -> (Symbol, Vec<Val>) {
//         let mut args: Vec<Val> = vec![e];
//         args.push_back(from.into_val(e));
//         args.push_back(spender.into_val(e));
//         args.push_back(amount.into_val(e));
//         args.push_back(expiration_ledger.into_val(e));
//         (Symbol::new(&e, APPROVE), args)
//     }

pub fn get_swap_call_data(
    e: &Env,
    token_in_id: Address,
    token_out_id: Address,
    amount_in: i128,
    amount_out: i128,
    to: Address,
    deadline: u64,
) -> (Symbol, Vec<Val>) {
    let path: Vec<Address> = vec![e, token_in_id.clone(), token_out_id.clone()];
    let mut args: Vec<Val> = vec![e];
    args.push_back(amount_out.into_val(e));
    args.push_back(amount_in.into_val(e));
    args.push_back(path.into_val(e));
    args.push_back(to.into_val(e));
    args.push_back(deadline.into_val(e));
    (Symbol::new(&e, SWAP_TOKENS_FOR_EXACT_TOKENS), args)
}

pub fn create_transfer_auth(
    e: &Env,
    amount_in: i128,
    token_in: Address,
    token_out: Address,
    to: Address,
) -> Vec<InvokerContractAuthEntry> {
    //   let factory = require_factory(e);

    let router_id = require_exchange_router(e);
    let pair = router_pair_for(e, &router_id, &token_in.clone(), &token_out.clone());
    // let pair = require_pair(e, factory.clone(), token_in.clone(), token_out.clone());

    // transfer arguments - used in soroswap router
    let mut args: Vec<Val> = vec![e];
    args.push_back(to.into_val(e));
    args.push_back(pair.into_val(e));
    args.push_back(amount_in.into_val(e));

    let mut sub_auth_vec = vec![&e];
    let pre_auth_entry = InvokerContractAuthEntry::Contract(SubContractInvocation {
        context: ContractContext {
            contract: token_in.clone(),
            fn_name: Symbol::new(&e, TRANSFER),
            args: args.clone(),
        },
        sub_invocations: vec![&e],
    });
    sub_auth_vec.push_back(pre_auth_entry);

    sub_auth_vec
}

pub fn create_swap_auth(
    e: &Env,
    router_id: Address,
    // amount_in: i128,
    // token_in: Address,
    // token_out: Address,
    // to: Address,
    args: Vec<Val>,
    sub_invocations: Vec<InvokerContractAuthEntry>,
) -> Vec<InvokerContractAuthEntry> {
    //   let factory = require_factory(e);

    // let router_id = require_exchange_router(e);
    //   let pair = router_pair_for(e,&router_id,  &token_in.clone(), &token_out.clone());
    // let pair = require_pair(e, factory.clone(), token_in.clone(), token_out.clone());

    // transfer arguments - used in soroswap router
    // let mut args: Vec<Val> = vec![e];
    // args.push_back(to.into_val(e));
    // args.push_back(pair.into_val(e));
    // args.push_back(amount_in.into_val(e));

    let mut sub_auth_vec = vec![&e];
    let pre_auth_entry = InvokerContractAuthEntry::Contract(SubContractInvocation {
        context: ContractContext {
            contract: router_id.clone(),
            fn_name: Symbol::new(&e, SWAP_TOKENS_FOR_EXACT_TOKENS),
            args: args.clone(),
        },
        sub_invocations, //vec![&e,transfer_auth],
    });
    sub_auth_vec.push_back(pre_auth_entry);

    sub_auth_vec
}

pub fn create_sub_auth(
    e: &Env,
    amount_in: i128,
    token_in: Address,
    token_out: Address,
    to: Address,
) -> Vec<InvokerContractAuthEntry> {
    let router_id = require_exchange_router(e);
    let pair = router_pair_for(e, &router_id, &token_in.clone(), &token_out.clone());

    // transfer arguments - used in soroswap router
    let mut args: Vec<Val> = vec![e];
    args.push_back(to.into_val(e));
    args.push_back(pair.into_val(e));
    args.push_back(amount_in.into_val(e));

    let mut sub_auth_vec = vec![&e];
    let pre_auth_entry = InvokerContractAuthEntry::Contract(SubContractInvocation {
        context: ContractContext {
            contract: token_in.clone(),
            fn_name: Symbol::new(&e, TRANSFER),
            args: args.clone(),
        },
        sub_invocations: vec![&e],
    });
    sub_auth_vec.push_back(pre_auth_entry);

    sub_auth_vec
}
