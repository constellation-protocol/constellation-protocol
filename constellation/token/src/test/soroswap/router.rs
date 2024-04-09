use soroban_sdk::Env;

mod router {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_router.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}

pub use router::SoroswapRouterClient;

pub fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    SoroswapRouterClient::new(e, &e.register_contract_wasm(None, router::WASM))
}
