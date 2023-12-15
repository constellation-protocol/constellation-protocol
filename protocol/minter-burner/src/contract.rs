#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, panic_with_error, symbol_short,
    token, Address, ConversionError, Env, InvokeError, Symbol,
};
use crate::error::{self, Error};
pub mod constellation_token {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}

#[contract]
pub struct MinterBurner;

// todo
// check amount
// check token address

#[contractimpl]
impl MinterBurner {
    pub fn mint(e: Env, to: Address, token_address: Address, amount: i128) {
        to.require_auth();
        if amount <= 0 {
            panic_with_error!(&e, Error::InvalidMintAmount);
        }
        let ctoken = constellation_token::Client::new(&e, &token_address);
        let mint_result = ctoken.try_mint(&to, &amount);
        let insufficient_balance = constellation_token::Error::InsufficientBalance as u32;
        match mint_result {
            Ok(result) => match result {
                Ok(()) => (),
                _ =>  panic_with_error!(&e, Error::ConversionError),
            },
            Err(error_reslt) => match error_reslt {
                insufficient_balance =>  panic_with_error!(&e, Error::InsufficientBalance),
                _ =>  panic_with_error!(&e, Error::InvokeError),
            },
        } 
    }
}
