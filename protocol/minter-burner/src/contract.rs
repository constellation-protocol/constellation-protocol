#![no_std]
use crate::error::{self, Error};
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, panic_with_error, symbol_short,
    token, Address, ConversionError, Env, InvokeError, Symbol,
};

pub(crate) use crate::token::constellation_token;

#[contract]
pub struct MinterBurner;

#[contractimpl]
impl MinterBurner {
    pub fn mint(
        e: Env,
        to: Address,
        constellation_token_address: Address,
        amount: i128,
    ) -> Result<(), Error> {
        to.require_auth();

        if amount <= 0 {
            return Err(Error::ZeroOrNegativeAmount);
        }

        let ctoken = constellation_token::Client::new(&e, &constellation_token_address);

        let components = ctoken.components();

        for c in components.iter() {
            let quantity = c.amount * amount; // unit * amount
            let _token = token::Client::new(&e, &c.address);
            _token.transfer_from(
                &e.current_contract_address(),
                &to,
                &constellation_token_address,
                &quantity,
            );
        }

        let mint_result = ctoken.try_mint(&to, &amount);
        let insufficient_balance = constellation_token::Error::InsufficientBalance as u32;

        match mint_result {
            Ok(result) => match result {
                Ok(()) => return Ok(()),
                Err(conversion_error) => panic_with_error!(&e, conversion_error),
            },
            Err(error_result) => match error_result {
                insufficient_balance => return Err(Error::InsufficientBalance),
                _ => panic_with_error!(&e, Error::ContractInvocationError),
            },
        }
    }

    pub fn burn(
        e: Env,
        from: Address,
        constellation_token_address: Address,
        amount: i128,
    ) -> Result<(), Error> {
        from.require_auth();

        if amount <= 0 {
            return Err(Error::ZeroOrNegativeAmount);
        }

        let ctoken = constellation_token::Client::new(&e, &constellation_token_address);

        let burn_result = ctoken.try_burn_from(&&e.current_contract_address(), &from, &amount);
        let insufficient_balance = constellation_token::Error::InsufficientBalance as u32;

        match burn_result {
            Ok(result) => match result {
                Ok(()) => return Ok(()),
                Err(conversion_error) => panic_with_error!(&e, conversion_error),
            },

            Err(error_result) => match error_result {
                insufficient_balance => return Err(Error::InsufficientBalance),
                _ => panic_with_error!(&e, Error::ContractInvocationError),
            },
        }

        ctoken.redeem(&from, &amount);

        Ok(())
    }
}
