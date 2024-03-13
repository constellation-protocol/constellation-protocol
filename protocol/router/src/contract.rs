#![no_std]
use crate::error::{self, Error};
use crate::event;
use crate::factory;
use crate::storage::{has_factory, read_factory, write_factory};
use crate::token as ctoken;
use soroban_sdk::auth::SubContractInvocation;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, panic_with_error, symbol_short,
    token, Address, BytesN, ConversionError, Env, InvokeError, String, Symbol, Vec,
};

#[contract]
pub struct Router;

#[contractimpl]
impl Router {
    /// Returns error if already initialized
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `factory` - Factory contract address
    pub fn initialize(e: Env, factory: Address) -> Result<(), Error> {
        if has_factory(&e) {
            return Err(Error::AlreadyInitalized);
        }
        write_factory(&e, &factory);
        event::initialize(&e, factory);
        Ok(())
    }

    /// Mints constellation token amount to specified address
    /// Returns error if already amount is 0 or negative
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `constellation_token_address` - Address of constellation token to mint
    /// - `amount` - Amount to mint
    ///
    /// Caller must possess balances of component tokens of the specified constellation token
    /// equal to or greater than the unit amount of the component token (of the constellation token) multiplied by
    /// the amount of constellation token to mint - see the lock function called in the mint function of the constellatio token
    ///
    /// Caller must also approve constellation token to spend each of the component tokens of the constellation token
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

    /// Burns constellation token amount and releases component tokens to the specified `from` address
    /// Returns error if already amount is 0 or negative
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `from` - Address to burn from
    /// - `constellation_token_address` - Address of constellation token to mint
    /// - `amount` - Amount to mint
    ///
    /// Caller must also approve router contract token to spend the constellation token
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

    /// creates new constellation token by calling factory
    /// Returns contellation token address. Returns error if number of components exceeds max if set
    ///
    /// # Arguments
    ///
    /// - `e` The runtime environment.
    /// - `decimal` Token decimal
    /// - `name` Name of token
    /// - `symbol` Symbol of token
    /// - `manager` Manages constellation token components and rebalancing
    /// - `components` Component tokens of this token
    /// - `amounts` Amounts of each componet token required to mint constellation token
    /// - `wasm_hash` Constellation token wasm hash
    /// - `salt` Unique salt
    #[allow(clippy::too_many_arguments)]
    pub fn create_token(
        e: Env,
        decimal: u32,
        name: String,
        symbol: String,
        manager: Address,
        components: Vec<Address>,
        amounts: Vec<i128>,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
    ) -> Result<Address, Error> { 
        let constellation_token_adddress = match read_factory(&e) {
            Some(factory) => factory::create(
                &e,
                decimal,
                name,
                symbol,
                &e.current_contract_address(),
                manager,
                components,
                amounts,
                factory,
                wasm_hash,
                salt,
            ),
        
            None => return Err(Error::RequiresFactory),
        };
        Ok(constellation_token_adddress)
    }

    /// Returns the address of factory contract
    pub fn get_factory_address(e: Env) -> Option<Address> {
        read_factory(&e)
    }
}
