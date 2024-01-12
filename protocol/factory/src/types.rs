#![no_std]
use soroban_env_common::declare_tag_based_object_wrapper;
use soroban_sdk::{contract, contracttype, Address, BytesN, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct CreateConstellationTokenArgs {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
    pub max_components: Option<u32>,
    pub admin: Address,
    pub manager: Address,
    pub components: Vec<Address>,
    pub amounts: Vec<i128>,
}
