use soroban_sdk::{contractclient, contractspecfn, token::Interface, Address, Env, String, Vec};
use soroban_sdk::{Symbol, Val};

use crate::error::Error;
use crate::storage::types::AllowanceValue;
use crate::storage::types::Component;

pub use ConstellationTokenInterfaceClient as MyClient;

#[contractclient(name = "ConstellationTokenInterfaceClient")]
pub trait ConstellationTokenInterface {
    fn initialize(
        e: Env,
        decimal: u32,
        components: Vec<Address>,
        amounts: Vec<i128>,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
    ) -> Result<(), Error>;

    fn mint(e: Env, to: Address, amount: i128) -> Result<(), Error>;

    fn redeem(e: Env, from: Address, amount: i128) -> Result<(), Error>;
    fn set_manager(e: Env, new_manager: Address) -> Result<(), Error>;

    fn get_components(e: Env) -> Vec<Component>;

    fn get_manager(e: Env) -> Option<Address>;
}

pub trait Module {
    fn add_module(e: Env, module: Address) -> Result<(), Error>;
    fn remove_module(e: Env, module: Address) -> Result<(), Error>;
    fn invoke(
        e: Env,
        caller_module_id: Address,
        target_exchange_id: Address,
        function_name: Symbol,
        args: Vec<Val>,
    ) -> Result<(), Error>;
}
