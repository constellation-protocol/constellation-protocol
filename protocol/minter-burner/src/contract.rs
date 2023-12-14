#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, symbol_short, token, Address,
    ConversionError, Env, InvokeError, Symbol,
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
    pub fn mint(env: Env, to: Address, token_address: Address, amount: i128) -> Result<u64, Error> {
        // Minter account needs to authorize
        to.require_auth();

        let ctoken = constellation_token::Client::new(&env, &token_address);

        let mint_result = ctoken.try_mint(&to, &amount);

        let InsufficientBalance = constellation_token::Error::InsufficientBalance as u32;

        match mint_result {
            Ok(result) => match result {
                Ok(result) => return Ok(1),                      //return Ok(()),
                Err(e) => return Ok(2), // return Err(Error::ConversionError),
            },

            Err(err) => {
                match err {
                    Ok(val) => {
                        // let s =  val.as_val();
                        // return Err(Error::MintError)
                        return Ok(val.to_val().get_payload());
                    }
                    _ => return Ok(4), //Val(4u64)
                }
            }
        }

        match mint_result {
            Ok(result) => match result  {
                Ok(result) => return Ok(()),
                Err(e) => return Err(Error::ConversionError),
            }

             Err(err) => {
                match err {
                    Ok(val) => {
                 
                        return Err(Error::MintError)
                    },
                    _ => Err(Error::ContractInvokeError),
                    // Err(InvokeError::Contract(InsufficientBalance)) => {
                    //     return Err(Error::MintInsufficientBalance);
                    // }
                    // Err(InvokeError::Abort) => {
                    //    return  Err(Error::Abort);
                    // }
                }
            }
            // Err(err) => {
            //     match err {
            //         Err(InvokeError::Contract(InsufficientBalance)) => {
            //             return Err(Error::MintInsufficientBalance);
            //         }
            //         _ => Err(Error::MintError),
            //     }
            // }
        }
    }
}
