use soroban_sdk::IntoVal;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, vec, Address, BytesN, Env, String, Symbol, Val, Vec,
};
use crate::require::{require_factory, require_pair};
use crate::storage::{factory, router};
use constellation_lib::traits::adapter::{self, dex};

static SWAP_EXACT_TOKENS_FOR_TOKENS: &'static str = "swap_exact_tokens_for_tokens";
static APPROVE: &'static str = "approve";
static TRANSFER: &'static str = "transfer";
#[contract]
pub struct SoroswapAdapter;

impl SoroswapAdapter {
    pub fn initialize(e: Env, router_id: Address, factory_id: Address) {
        if router::has_router(&e) {
            panic!("already initialized");
        }
        router::write_router(&e, &router_id);
        factory::write_factory(&e, &factory_id);
    }
}

impl dex::Interface for SoroswapAdapter {
    fn get_approve_call_data(
        e: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> (Symbol, Vec<Val>) {
        let mut args: Vec<Val> = vec![e];
        args.push_back(from.into_val(e));
        args.push_back(spender.into_val(e));
        args.push_back(amount.into_val(e));
        args.push_back(expiration_ledger.into_val(e));
        (Symbol::new(&e, APPROVE), args)
    }
    fn get_swap_call_data(
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
        args.push_back(amount_in.into_val(e));
        args.push_back(amount_out.into_val(e));
        args.push_back(path.into_val(e));
        args.push_back(to.into_val(e));
        args.push_back(deadline.into_val(e));
        (Symbol::new(&e, SWAP_EXACT_TOKENS_FOR_TOKENS), args)
    }

    fn create_sub_auth(
        e: &Env,
        amount_in: i128,
        token_in: Address,
        token_out: Address,
        constellation_token_id: Address,
    ) -> Vec<InvokerContractAuthEntry>  {
        let factory =  require_factory(e);

        let pair = require_pair(e, factory.clone(), token_in.clone(), token_out.clone());

        // transfer arguments - used in soroswap router 
        let mut args: Vec<Val> = vec![e];
        args.push_back(constellation_token_id.into_val(e));
        args.push_back(pair.into_val(e));
        args.push_back(amount_in.into_val(e));

        let mut sub_auth_vec = vec![&e];
        let pre_auth_entry = InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: constellation_token_id.clone(),
                fn_name: Symbol::new(&e, TRANSFER),
                args: args.clone(),
            },
            sub_invocations: vec![&e],
        });
        sub_auth_vec.push_back(pre_auth_entry);
       
       sub_auth_vec
    }
}
