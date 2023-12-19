use soroban_sdk::{
    contract, contracterror, contractimpl, log, symbol_short, token, Address, Env, String, Symbol,
    Vec, panic_with_error,
};

// pub type Result<T> = core::result::Result<T, Error>;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitalized = 400,
    // expiration_ledger is less than ledger seq when amount
    ExpirationLedgerLessThanLedgerSequence = 401,
    ZeroOrNegativeAmountNotAllowed = 402,
    ComponentsAmountsLengthMismatch = 403,
    ZeroAmount = 404,
    ZeroComponents = 405,
    ZeroLength = 406,
    AddressIndex = 407,
    AmountIndex = 408,
    InsufficientAllowance = 500,
    InsufficientBalance = 501,
}

pub fn check_zero_or_negative_amount(e: &Env ,amount: i128) {
    if amount <= 0 {
        panic_with_error!(&e, Error::ZeroOrNegativeAmountNotAllowed);
    }
}
