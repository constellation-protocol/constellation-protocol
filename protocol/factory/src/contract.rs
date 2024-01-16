use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
use crate::error::Error;
use crate::event;
use crate::helpers::deploy;
use crate::storage::max_components::{read_max_components, write_max_components};
use crate::storage::token_list::{read_token_list, write_token_list};
use crate::storage::DataKey;
use crate::token::{constellation_token, initialize_token};
use crate::storage::admin::{has_administrator, read_administrator, write_administrator};
#[contract]
pub struct Factory {}

#[contractimpl]
impl Factory {

    pub fn initialize(e: Env, admin: Address) -> Result<(), Error> {
        if has_administrator(&e) {
            return Err(Error::AlreadyInitialized);
        }

        write_administrator(&e, &admin);

        Ok(())
    }

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
    pub fn set_max_components(e: Env, max_components: u32) -> Result<(), Error> {

        match read_administrator(&e) {
            Some(admin) => admin.require_auth(),
            None => return Err(Error::RequiresAdministrator)
        }

        if max_components == 0 {
            return Err(Error::ZeroValue);
        }
        write_max_components(&e, max_components);
        event::set_max_components(&e, max_components);
        Ok(())
    }
    pub fn get_token_list(e: Env) -> Vec<Address> {
        read_token_list(&e)
    }
    pub fn get_max_components(e: Env) -> Option<u32> {
        read_max_components(&e)
    }
}
