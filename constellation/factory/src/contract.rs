use crate::error::Error;
use crate::event;
use crate::helpers::deploy;
use crate::require::require_constellation_wasm_hash;
use crate::storage::admin::{has_administrator, read_administrator, write_administrator};
use crate::storage::constellation_token_hash::{read_constellation_hash, write_constellation_hash};
use crate::storage::deployments_count::{read_deployment_count, write_deployment_count};
use crate::storage::max_components::{read_max_components, write_max_components};
use crate::storage::token_list::{read_token_list, write_token_list};
use crate::storage::DataKey;
use crate::token::{constellation_token, initialize_token};
use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Bytes, BytesN, Env, String, Vec};
#[contract]
pub struct Factory {}

#[contractimpl]
impl Factory {
    /// Returns error if already initialized
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `admin` - Address of contract administrator
    pub fn initialize(e: Env, admin: Address, constellation_token_wasm_hash: BytesN<32>) -> Result<(), Error> {
        if has_administrator(&e) {
            return Err(Error::AlreadyInitialized);
        }

        write_administrator(&e, &admin);
        write_constellation_hash(&e, &constellation_token_wasm_hash);

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
    ) -> Result<Address, Error> {
        if let Some(max) = read_max_components(&e) {
            if components.len() > max {
                return Err(Error::ExceedsMaxComponents);
            }
        }

        let next_deployment: u64 = Self::get_or_set_deployment_count(&e);

        let wasm_hash = require_constellation_wasm_hash(&e)?;

        let salt = Self::create_Salt(&e, &next_deployment, &wasm_hash)?;

        let address = deploy(&e, deployer, wasm_hash, salt);

        initialize_token(
            &e, &address, decimal, name, symbol, admin, manager, components, amounts,
        );
        write_token_list(&e, address.clone());

        Self::store_next_deployment_count(&e, next_deployment);

        event::create(&e, &address);
        Ok(address)
    }

    /// sets maxumum number of component tokens allowed when creating a new constellation token
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

    // sets constellation token hash
    ///
    /// # Arguments
    /// - `e` - The runtime environment.
    /// - `value` - Value of max components
    pub fn set_constellation_token(e: Env, hash: BytesN<32>) -> Result<(), Error> {
        match read_administrator(&e) {
            Some(admin) => admin.require_auth(),
            None => return Err(Error::RequiresAdministrator),
        }

        write_constellation_hash(&e, &hash);
        event::set_constellation_token(&e, hash);
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

    fn store_next_deployment_count(
        e: &Env,
        deployment_count: u64, 
    )  {
        let next_deployment_count = deployment_count +  1;
        write_deployment_count(e, next_deployment_count);
    }

    fn create_Salt(
        e: &Env,
        next_deployment: &u64,
        wasm_hash: &BytesN<32>,
    ) -> Result<BytesN<32>, Error> {
        let mut salt = Bytes::new(e);
        salt.append(&wasm_hash.clone().to_xdr(e));
        salt.append(&next_deployment.to_xdr(e));

        Ok(e.crypto().sha256(&salt))
    }

    fn get_or_set_deployment_count(e: &Env) -> u64 {
        let count = match read_deployment_count(e) {
            Some(count) => count,
            None => {
                let first = 1;
                write_deployment_count(e, first);
                first
            }
        };
        count
    }
}
