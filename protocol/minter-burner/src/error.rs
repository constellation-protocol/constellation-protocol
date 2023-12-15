use soroban_sdk::{
    contract, contracterror, contractimpl, log, symbol_short, token, Address, Env, String, Symbol,
    Val, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InsufficientBalance = 1,
    InvalidMintAmount = 2,
    ZeroComponents = 3,
    ConversionError = 4,
    InvokeError = 5,
}
