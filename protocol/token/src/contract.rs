use crate::component::read_components;
use crate::component::write_components;
use crate::error::check_zero_or_negative_amount;
use crate::error::Error;
use crate::manager::{read_manager, write_manager};
use crate::admin::read_administrator;
use crate::admin::{has_administrator, write_administrator};
use crate::allowance::*;
use crate::balance::*;
use crate::metadata::*;
use crate::metadata::{
    read_decimal, read_name, read_symbol, write_metadata,
};
use crate::storage_types::{
    INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
};
use crate::types::Component;
use soroban_sdk::panic_with_error;
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, token, token::Interface, Address, Env,
    String, Symbol, Vec,
};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

#[contract]
pub struct ConstellationToken;

#[contractimpl]
impl ConstellationToken {
    //////////////////////////////////////////////////////////////////
    ///////// mutable functions //////////////////////////////////////
    //////////////////////////////////////////////////////////////////
    pub fn initialize(
        e: Env,
        decimal: u32,
        components: Vec<Address>,
        amounts: Vec<i128>,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
    ) -> Result<(), Error> {
        if has_administrator(&e) {
          panic_with_error!(&e, Error::AlreadyInitalized);
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

        Ok(())
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_zero_or_negative_amount(&e, amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    //////////////////////////////////////////////////////////////////
    ///////// Read Only functions ////////////////////////////////////
    //////////////////////////////////////////////////////////////////

    pub fn admin(e: Env) -> Address {
        read_administrator(&e)
    }

    pub fn components(e: Env) -> Vec<Component> {
        read_components(&e)
    }

    pub fn manager(e: Env) -> Address {
        read_manager(&e)
    }
}

#[contractimpl]
impl token::Interface for ConstellationToken {
    // fn burn(e: Env, from: Address, amount: i128) {
    //     check_zero_or_negative_amount(&e, amount);
    //     let admin = read_administrator(&e);
    //     admin.require_auth();

    //     if read_balance(&e, from.clone()) < amount {
    //        panic_with_error!(&e, Error::InsufficientBalance);
    //     }

    //     e.storage()
    //         .instance()
    //         .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

    //     spend_balance(&e, from.clone(), amount);

    //     let components = read_components(&e);
    //     for c in components.iter() {
    //         let _token = token::Client::new(&e, &c.address);
    //         _token.transfer(
    //             &e.current_contract_address(),
    //             &from.clone(),
    //             &(c.amount * amount),
    //         );
    //     }
    //     TokenUtils::new(&e).events().burn(from, amount);
    // }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount)
    }
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_zero_or_negative_amount(&e, amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
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
