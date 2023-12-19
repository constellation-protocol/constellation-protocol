#![no_std]
use crate::error::{self, Error};
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, panic_with_error, symbol_short,
    token, Address, ConversionError, Env, InvokeError, Symbol,
};
pub mod constellation_token {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
}

#[contract]
pub struct MinterBurner;

#[contractimpl]
impl MinterBurner {
    pub fn mint(e: Env, to: Address, token_address: Address, amount: i128) -> Result<(), Error> {
        to.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidMintAmount);
        }

        let ctoken = constellation_token::Client::new(&e, &token_address);

        let components = ctoken.components();

        if components.len() == 0 {
            panic_with_error!(&e, Error::ZeroComponents)
        }

        for c in components.iter() {
            let quantity = c.amount * amount; // unit * amount
            let _token = token::Client::new(&e, &c.address);
            // if _token.balance(&to) < quantity {
            //     panic_with_error!(&e, Error::InsufficientBalance);
            // }
            _token.transfer_from(
                &e.current_contract_address(),
                &to,
                &token_address,
                &quantity,
            );
        }

        let mint_result = ctoken.try_mint(&to, &amount);
        let insufficient_balance = constellation_token::Error::InsufficientBalance as u32;

        match mint_result {
            Ok(result) => match result {
                Ok(()) => return Ok(()),
                _ => return Err( Error::ConversionError), 
            },
            Err(error_reslt) => match error_reslt {
                insufficient_balance => return Err(Error::InsufficientBalance), 
                _ => return Err(Error::InvokeError),
            },
        }

        Ok(())
    }

    fn burn(e: Env, spender: Address, from: Address, amount: i128) {}
}
