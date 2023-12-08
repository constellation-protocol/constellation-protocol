use soroban_sdk::{
    contract, contracterror, contractimpl, log, symbol_short, token, Address, Env, String, Symbol,
    Vec,
};

// pub type Result<T> = core::result::Result<T, Error>;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// #[repr(u32)]
pub enum Error {
    MintInsufficientBalance = 1,
    MintError = 2
    
}
 