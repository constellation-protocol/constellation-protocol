
use soroban_sdk::{symbol_short, Address,Symbol, Env, String,IntoVal};

pub(crate) fn create(e: &Env, address: &Address) {
    let topics = (symbol_short!("create"),);
    e.events().publish(topics, address)
}

pub(crate) fn set_max_components(e: &Env, max_components: u32) {
    let topics = (Symbol::new(e, "set_max_components"),);
    e.events().publish(topics, max_components)
}