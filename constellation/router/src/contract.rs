use crate::error::{self, Error};
use crate::event;
use crate::factory;
use crate::helper::{
     get_required_amount_token_in as _get_required_amount_token_in,
    refund_unspent, swap_exact_tokens_for_tokens, swap_tokens_for_exact_tokens,
};
use crate::require::require_exchange_router;
use crate::soroswap_router;
use crate::storage::{
    has_factory, read_exchange_router, read_factory, write_exchange_router,
    write_factory,
};
use crate::token as ctoken;
use crate::token::constellation_token::Component;
use constellation_lib::traits::adapter::dex;
use soroban_sdk::auth::SubContractInvocation;
use soroban_sdk::{
    auth::InvokerContractAuthEntry, contract, contracterror, contractimpl, contracttype, log,
    panic_with_error, symbol_short, token, vec, xdr, Address, BytesN, ConversionError, Env,
    InvokeError, String, Symbol, Val, Vec,
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
    pub fn initialize(
        e: Env,
        factory: Address,
        soroswap_router: Address,
    ) -> Result<(), Error> {
        if has_factory(&e) {
            return Err(Error::AlreadyInitalized);
        }
        write_factory(&e, &factory);
        write_exchange_router(&e, &soroswap_router);
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
    /// - `mint_amount` -  Amount of  constellation tokens to mint
    /// - `amount_in` - amount of input token
    /// - `token_in` - Address of input token
    /// - `to` - Address to receive constellation token
    /// - `constellation_token_id` Constellation token address
    /// - `deadline` swap deadline
    ///
    /// Caller must possess balances of component tokens of the specified constellation token
    /// equal to or greater than the unit amount of the component token (of the constellation token) multiplied by
    /// the amount of constellation token to mint - see the lock function called in the mint function of the constellatio token
    ///
    pub fn mint_exact_tokens(
        e: Env,
        mint_amount: i128,
        amount_in: i128,
        token_in: Address,
        to: Address,
        constellation_token_id: Address,
        deadline: u64,
    ) -> Result<i128, Error> {
        to.require_auth();

        let router_id = require_exchange_router(&e);
        // transfers token in to the router
        token::Client::new(&e, &token_in).transfer_from(
            &e.current_contract_address(),
            &to,
            &e.current_contract_address(),
            &amount_in,
        );

        let components = ctoken::get_components(&e, &constellation_token_id);

        let (total_token_in_amount, token_amounts_in) =
            _get_required_amount_token_in(&e, &token_in, mint_amount, &components)?;

        if total_token_in_amount > amount_in {
            return Err(Error::InsufficientInputAmount);
        }
       
        token::Client::new(&e, &token_in).approve(&e.current_contract_address(),&router_id, &amount_in,  &(e.ledger().sequence() + 1000u32));
        
        // swaps token_in for the component tokens
        let mut total_spent = swap_tokens_for_exact_tokens(
            &e,
            &mint_amount,
            &token_in,
            &e.current_contract_address(),
            &router_id,
            &token_amounts_in,
            &components,
            &constellation_token_id,
            deadline,
        )?;
        
        // mints the constellation token
        ctoken::mint(&e, &to, mint_amount, &constellation_token_id);

        let refund = amount_in - total_spent;

        refund_unspent(&e, refund, &token_in, &to, deadline);

        event::mint_exact_constellation(&e, to, mint_amount, refund);
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
            let amount_in = c.unit *  amount;
            soroswap_router::swap_exact_tokens_for_tokens(
                &e,
                &router_id,
                amount_in,
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
            ),

            None => return Err(Error::RequiresFactory),
        };
        Ok(constellation_token_adddress)
    }

    /// Returns the address of factory contract
    pub fn get_factory_address(e: Env) -> Option<Address> {
        read_factory(&e)
    }

    pub fn get_required_amount_token_in(
        e: Env,
        token_in: Address,
        mint_amount: i128,
        components: Vec<Component>,
    ) -> Result<(i128, Vec<i128>), Error> {
        _get_required_amount_token_in(&e, &token_in, mint_amount, &components)
    }

    pub fn invoke(
        e: Env,
        module_id: Address,
        target_id: Address,
        call_data: (Symbol, Vec<Val>),
        auth_entries: Vec<InvokerContractAuthEntry>,
    ) -> Result<(), Error> {
        //  module_id.require_auth();

        let (function, args) = call_data;
        e.authorize_as_current_contract(auth_entries);
        e.invoke_contract::<Val>(&target_id, &function, args);
        Ok(())
    }
}
