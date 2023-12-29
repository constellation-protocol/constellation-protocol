#![no_std]

mod admin;
mod allowance;
mod balance;
mod metadata;
mod storage_types;

mod component;
mod contract;
pub mod error;
mod event;
mod helpers;
mod manager;
#[cfg(test)]
mod test;
pub mod traits;

pub use crate::contract::{ConstellationToken, ConstellationTokenClient};
