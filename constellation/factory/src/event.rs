use soroban_sdk::{symbol_short, Address, BytesN, Env, IntoVal, String, Symbol};
pub(crate) fn create(e: &Env, address: &Address) {
    let topics = (symbol_short!("create"),);
    e.events().publish(topics, address)
}
pub(crate) fn set_max_components(e: &Env, max_components: u32) {
    let topics = (Symbol::new(e, "set_max_components"),);
    e.events().publish(topics, max_components)
}

pub(crate) fn set_constellation_token(e: &Env, hash: BytesN<32>) {
    let topics = (Symbol::new(e, "set_constellation_token"),);
    e.events().publish(topics, hash)
}
