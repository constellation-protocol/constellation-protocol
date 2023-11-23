use crate::component::read_components;
use crate::component::write_components;
use crate::error::check_nonnegative_amount;
use crate::error::Error;
use crate::manager::write_manager;
use crate::token_interface_storage::admin::read_administrator;
use crate::token_interface_storage::admin::{has_administrator, write_administrator};
use crate::token_interface_storage::allowance::*;
use crate::token_interface_storage::balance::*;
use crate::token_interface_storage::metadata::*;
use crate::token_interface_storage::metadata::{
    read_decimal, read_name, read_symbol, write_metadata,
};
use crate::token_interface_storage::storage_types::{
    INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
};
use crate::types::Component;
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, token, token::Interface, Address, Env,
    String, Symbol, Vec,
};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

#[contract]
pub struct ConstellationToken;

#[contractimpl]
impl ConstellationToken {
    pub fn initialize(
        e: Env,
        decimal: u32,
        components: Vec<Address>,
        amounts: Vec<i128>,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
    ) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        write_manager(&e, &manager);
        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );
        write_components(&e, components, amounts);
    }

    pub fn admin(e: Env) -> Address {
        read_administrator(&e)
    }

    pub fn components(e: Env) -> Vec<Component> {
        read_components(&e)
    }

    pub fn mint(env: Env, to: Address, amount: i128) {}
}

#[contractimpl]
impl token::Interface for ConstellationToken {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn spendable_balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
