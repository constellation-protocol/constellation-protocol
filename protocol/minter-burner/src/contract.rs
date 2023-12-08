#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, symbol_short, token, Address, Env,
    Symbol,
};

use crate::error::Error;

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
    pub fn mint(env: Env, from: Address, to: Address, token_address: Address, amount: i128) -> Result<(), Error> {
        // Minter account needs to authorize
        from.require_auth();

        let ctoken = constellation_token::Client::new(&env, &token_address);

        let mint_result: Result<Result<(), soroban_sdk::ConversionError>, Result<soroban_sdk::Error, std::convert::Infallible>> = ctoken.try_mint(&to, &amount);
    
        match mint_result {
            Ok(ok) => {
               Ok(())
            },
            Err(err) => {
                 match err {
                //    Err(ctoken::Error::InsufficientBalance) => {
                //      return error::MintInsufficientBalance
                //    },
                   _ => Err(Error::MintError),
                }
            }
        }
    }
}
