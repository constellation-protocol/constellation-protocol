use soroban_sdk::{token::Interface, Address, Env, String, Vec};

use crate::error::Error;
use crate::storage_types::AllowanceValue;
use crate::storage_types::Component;
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
