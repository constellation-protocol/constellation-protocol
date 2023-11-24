use soroban_sdk::{
    contract, contracterror, contractimpl, log, symbol_short, token, Address, Env, String, Symbol,
    Vec,
};

//  pub type Result<T> = core::result::Result<T, Error>;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InsufficientBalance = 1,
}

pub fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

pub fn ensure_equal_lengths(a: u32, b: u32, a_str: &'static str, b_str: &'static str) {
    if a != b {
        panic!("{} / {} lengths mismatch", a_str, b_str);
    }
}

pub fn ensure_none_zero(a: u32, a_str: &'static str) {
    if a == 0 {
        panic!("{} cannot be 0", a_str);
    }
}
