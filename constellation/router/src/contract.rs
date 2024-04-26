use crate::error::{self, Error};
use crate::event;
use crate::factory;
use crate::helper::{
    get_base_token_amount_in, get_required_amount_token_in, refund_unspent,
    swap_tokens_for_exact_tokens,
};
use crate::require::{require_exchange_router, require_xlm};
use crate::storage::{
    has_factory, read_exchange_router, read_factory, read_xlm, write_exchange_router,
    write_factory, write_xlm, xlm,
};
use crate::token as ctoken;
use crate::token::constellation_token::Component;
use soroban_sdk::auth::SubContractInvocation;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, panic_with_error, symbol_short,
    token, vec, Address, BytesN, ConversionError, Env, InvokeError, String, Symbol, Vec,
};

use crate::soroswap_router;

#[contract]
pub struct Router;

#[contractimpl]
impl Router {
    /// Returns error if already initialized
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `factory` - Factory contract address
    pub fn initialize(
        e: Env,
        factory: Address,
        soroswap_router: Address,
        xlm: Address,
    ) -> Result<(), Error> {
        if has_factory(&e) {
            return Err(Error::AlreadyInitalized);
        }
        write_factory(&e, &factory);
        write_exchange_router(&e, &soroswap_router);
        write_xlm(&e, &xlm);
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

        ctoken::mint(&e, &to, amount, &constellation_token_address);
        Ok(())
    }

    /// Mints constellation token amount to specified address
    /// Returns error if already amount is 0 or negative
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `amount_in` - Address of constellation token to mint
    /// - `amount` - Amount to mint
    ///
    /// Caller must possess balances of component tokens of the specified constellation token
    /// equal to or greater than the unit amount of the component token (of the constellation token) multiplied by
    /// the amount of constellation token to mint - see the lock function called in the mint function of the constellatio token
    ///
    /// Caller must also approve constellation token to spend each of the component tokens of the constellation token
    pub fn mint_exact_constellation(
        e: Env,
        amount_in: i128,
        amount: i128,
        token_in: Address,
        constellation_token_id: Address,
        to: Address,
        deadline: u64,
    ) -> Result<i128, Error> {
        to.require_auth();

        let router_id = require_exchange_router(&e);

        token::Client::new(&e, &token_in).transfer_from(
            &e.current_contract_address(),
            &to,
            &e.current_contract_address(),
            &amount_in,
        );

        let xlm_id = require_xlm(&e);

        let components = ctoken::get_components(&e, &constellation_token_id);

        let (total_token_in_amount, token_amounts_in) =
            get_required_amount_token_in(&e, &token_in, amount, &components)?;

        let amount_in = get_base_token_amount_in(
            &e,
            &router_id,
            amount_in,
            0,
            &token_in,
            &xlm_id,
            &e.current_contract_address(),
            deadline,
        )?;

        if total_token_in_amount > amount_in {
            return Err(Error::InsufficientInputAmount);
        }

        let mut total_spent = swap_tokens_for_exact_tokens(
            &e,
            &router_id,
            &token_amounts_in,
            i128::MAX,
            &xlm_id,
            &to,
            deadline,
            &components,
        )?;

        ctoken::mint(&e, &to, amount, &constellation_token_id);

        let amount_unspent = amount_in - total_spent;

        let refund = refund_unspent(
            &e,
            &router_id,
            amount_unspent,
            0,
            &xlm_id,
            &token_in,
            &to,
            deadline,
        )?;

        event::mint_exact_constellation(&e, to, amount, refund);

        Ok(refund)
    }

    pub fn redeem_into(
        e: Env,
        to: Address,
        amount: i128,
        constellation_token: Address,
        redeem_token: Address,
        deadline: u64,
    ) -> Result<(), Error> {
        to.require_auth();

        if amount <= 0 {
            return Err(Error::ZeroOrNegativeAmount);
        }

        let router_id = &require_exchange_router(&e);

        ctoken::redeem(
            &e,
            &to,
            &e.current_contract_address(),
            amount,
            &constellation_token,
        );

        let components = ctoken::get_components(&e, &constellation_token);

        for c in components.iter() {
            let balance = token::Client::new(&e, &c.address).balance(&e.current_contract_address());
            soroswap_router::swap_exact_tokens_for_tokens(
                &e,
                router_id,
                balance,
                0,
                &c.address,
                &redeem_token,
                &to,
                deadline,
            );
        }

        event::redeem_into(&e, to, redeem_token, constellation_token, amount);
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

        ctoken::redeem(&e, &from, &from, amount, &constellation_token_address);
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
            Some(_factory) => factory::create(
                &e,
                decimal,
                name,
                symbol,
                &e.current_contract_address(),
                manager,
                components,
                amounts,
                _factory,
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
