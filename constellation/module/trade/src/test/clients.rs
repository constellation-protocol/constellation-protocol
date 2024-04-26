use crate::contract::{Trade, TradeClient};
use soroban_sdk::{Address, BytesN, Env};

pub use adapter::TradeAdapterClient;
pub use constellation_token::ConstellationTokenClient;
pub use factory::SoroswapFactoryClient;
pub use registry::RegistryClient;
pub use router::SoroswapRouterClient;
pub use token::TokenClient;

mod constellation_token {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
    pub type ConstellationTokenClient<'a> = Client<'a>;
}

pub mod adapter {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        // file = "../../../libs/constellation_adapter_soroswap.wasm"
        file = "../../../target/wasm32-unknown-unknown/release/constellation_adapter_soroswap.wasm"
    );
    pub type TradeAdapterClient<'a> = Client<'a>;
}

pub mod registry {
    soroban_sdk::contractimport!(
        file = "../../../target/wasm32-unknown-unknown/release/constellation_registry.wasm"
    );
    pub type RegistryClient<'a> = Client<'a>;
}

mod router {
    soroban_sdk::contractimport!(file = "../../../libs/soroswap_router.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}

mod factory {
    soroban_sdk::contractimport!(file = "../../../libs/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}

pub mod token {
    soroban_sdk::contractimport!(file = "../../../libs/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

pub fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "../../../libs/soroswap_pair.wasm");
    e.deployer().upload_contract_wasm(WASM)
}

pub fn create_trade_module<'a>(e: &Env) -> TradeClient<'a> {
    let contract_id = &e.register_contract(None, Trade {});
    TradeClient::new(e, contract_id)
}

pub fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    SoroswapRouterClient::new(e, &e.register_contract_wasm(None, router::WASM))
}

pub fn create_soroswap_factory<'a>(e: &Env) -> SoroswapFactoryClient<'a> {
    SoroswapFactoryClient::new(e, &e.register_contract_wasm(None, factory::WASM))
}

pub fn create_constellation_token<'a>(e: &Env) -> ConstellationTokenClient<'a> {
    let contract_id = &e.register_contract_wasm(None, constellation_token::WASM);
    ConstellationTokenClient::new(e, contract_id)
}

pub fn create_adapter<'a>(e: &Env) -> adapter::Client<'a> {
    adapter::Client::new(e, &e.register_contract_wasm(None, adapter::WASM))
}

pub fn create_registry<'a>(e: &Env) -> registry::Client<'a> {
    registry::Client::new(e, &e.register_contract_wasm(None, registry::WASM))
}

pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}
