use crate::error::Error;
use crate::event;
use crate::helpers::deploy;
use crate::storage::admin::{has_administrator, read_administrator, write_administrator};
use crate::storage::max_components::{read_max_components, write_max_components};
use crate::storage::token_list::{read_token_list, write_token_list};
use crate::storage::DataKey;
use crate::token::{constellation_token, initialize_token};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
#[contract]
pub struct Factory {}

#[contractimpl]
impl Factory {
    /// Returns error if already initialized
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `admin` - Address of contract administrator
    pub fn initialize(e: Env, admin: Address) -> Result<(), Error> {
        if has_administrator(&e) {
            return Err(Error::AlreadyInitialized);
        }

        write_administrator(&e, &admin);

        Ok(())
    }

    /// creates new constellation token
    /// Returns contellation token address. Returns error if number of components exceeds max if set
    ///
    /// # Arguments
    ///
    /// - `e` The runtime environment.
    /// - `decimal` Token decimal
    /// - `name` Name of token
    /// - `symbol` Symbol of token
    /// - `admin` Token administrator
    /// - `manager` Manages constellation token components and rebalancing
    /// - `components` Component tokens of this token
    /// - `amounts` Amounts of each componet token required to mint constellation token
    /// - `deployer` Address which deploys the new constellation token
    /// - `wasm_hash` Constellation token wasm hash
    /// - `salt` Unique salt
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        e: Env,
        decimal: u32,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
        components: Vec<Address>,
        amounts: Vec<i128>,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
    ) -> Result<Address, Error> {
        if let Some(max) = read_max_components(&e) {
            if components.len() > max {
                return Err(Error::ExceedsMaxComponents);
            }
        }
        let address = deploy(&e, deployer, wasm_hash, salt);
        initialize_token(
            &e, &address, decimal, name, symbol, admin, manager, components, amounts,
        );
        write_token_list(&e, address.clone());
        event::create(&e, &address);
        Ok(address)
    }

    /// sets maxumum number of component tokens allowed when creating a new constellation tokekn
    /// returns error if Administrator is not set
    /// returns error if value is zero
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `value` - Value of max components
    pub fn set_max_components(e: Env, value: u32) -> Result<(), Error> {
        match read_administrator(&e) {
            Some(admin) => admin.require_auth(),
            None => return Err(Error::RequiresAdministrator),
        }

        if value == 0 {
            return Err(Error::ZeroValue);
        }
        write_max_components(&e, value);
        event::set_max_components(&e, value);
        Ok(())
    }

    /// Returns list of created component tokens
    pub fn get_token_list(e: Env) -> Vec<Address> {
        read_token_list(&e)
    }

    /// Returns maximum number of component tokens allowed when creating a new constellation tokekn
    pub fn get_max_components(e: Env) -> Option<u32> {
        read_max_components(&e)
    }
}
