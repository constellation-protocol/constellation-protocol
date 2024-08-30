use soroban_sdk::{Address, BytesN, Env};

pub use crate::contract::RouterClient;
pub use adapter::TradeAdapterClient;
pub use constellation_token::ConstellationTokenClient;
pub use pair::PairClient;
pub use registry::RegistryClient;
pub use router::SoroswapRouterClient;
pub use s_factory::SoroswapFactoryClient;
pub use token::TokenClient;

pub mod constellation_token {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_token.wasm"
    );
    pub type ConstellationTokenClient<'a> = Client<'a>;
}

pub fn upload_constellation_token(e: &Env) -> BytesN<32> {
    e.deployer().upload_contract_wasm(constellation_token::WASM)
}

pub mod adapter {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        // file = "../../../libs/constellation_adapter_soroswap.wasm"
        file = "../../target/wasm32-unknown-unknown/release/constellation_adapter_soroswap.wasm"
    );
    pub type TradeAdapterClient<'a> = Client<'a>;
}

pub mod registry {
    use soroban_sdk::auth::InvokerContractAuthEntry;
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/constellation_registry.wasm"
    );
    pub type RegistryClient<'a> = Client<'a>;
}

mod router {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_router.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}

mod s_factory {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

pub mod pair {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_pair.wasm");
    pub type PairClient<'a> = Client<'a>;
}

pub fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_pair.wasm");
    e.deployer().upload_contract_wasm(WASM)
}

pub fn create_soroswap_pair_contract<'a>(e: &Env) -> PairClient<'a> {
    PairClient::new(e, &e.register_contract_wasm(None, pair::WASM))
}

pub fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    SoroswapRouterClient::new(e, &e.register_contract_wasm(None, router::WASM))
}

pub fn create_soroswap_factory<'a>(e: &Env) -> SoroswapFactoryClient<'a> {
    SoroswapFactoryClient::new(e, &e.register_contract_wasm(None, s_factory::WASM))
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

pub fn create_router<'a>(e: &Env) -> RouterClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::Router {});
    let ct: RouterClient<'_> = RouterClient::new(e, contract_id);
    ct
}

pub fn create_factory<'a>(e: &Env) -> crate::factory::constellation_factory::Client<'a> {
    let contract_id = &e.register_contract_wasm(None, crate::factory::constellation_factory::WASM);
    let factory = crate::factory::constellation_factory::Client::new(e, contract_id);
    factory
}
