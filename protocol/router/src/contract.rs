#![no_std]
use crate::error::{self, Error};
use crate::event;
use crate::factory;
use crate::storage::{has_factory, read_factory, write_factory};
use crate::token as ctoken;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, panic_with_error, symbol_short,
    token, Address, BytesN, ConversionError, Env, InvokeError, String, Symbol, Vec,
};

#[contract]
pub struct Router;

#[contractimpl]
impl Router {
    pub fn initialize(e: Env, factory: Address) -> Result<(), Error> {
        if has_factory(&e) {
            return Err(Error::AlreadyInitalized);
        }
        write_factory(&e, &factory);
        event::initialize(&e, factory);
        Ok(())
    }
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

        ctoken::mint(&e, to, amount, constellation_token_address);
        Ok(())
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

        ctoken::redeem(&e, from, amount, constellation_token_address);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_token(
        e: Env,
        decimal: u32,
        name: String,
        symbol: String,
        manager: Address,
        components: Vec<Address>,
        amounts: Vec<i128>,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
    ) -> Address {
        factory::create(
            &e,
            decimal,
            name,
            symbol,
            &e.current_contract_address(),
            manager,
            components,
            amounts,
            deployer,
            wasm_hash,
            salt,
        )
    }
}
