extern crate std;

use super::add_liquidity::add_liquidity;
use super::clients::{
    constellation_token as constellation_token_mod, create_adapter, create_constellation_token,
    create_factory, create_registry, create_router, create_soroswap_factory,
    create_soroswap_pair_contract, create_soroswap_router, create_token_contract,
    pair_contract_wasm, registry, upload_constellation_token, ConstellationTokenClient,
    RegistryClient, RouterClient, SoroswapFactoryClient, SoroswapRouterClient, TokenClient,
    TradeAdapterClient,
};
use crate::factory::ConstellationFactoryClient;

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String, Vec};
pub type Tokens<'a> = (
    TokenClient<'a>,
    TokenClient<'a>,
    TokenClient<'a>,
    TokenClient<'a>,
);

pub struct TradeTest<'a> {
    pub env: Env,
    pub user: Address,
    pub admin: Address,
    pub adapter: TradeAdapterClient<'a>,
    pub registry: RegistryClient<'a>,
    pub router: RouterClient<'a>,
    pub s_router: SoroswapRouterClient<'a>,
    pub s_factory: SoroswapFactoryClient<'a>,
    pub factory: ConstellationFactoryClient<'a>,
    pub constellation_token: ConstellationTokenClient<'a>,
    pub tokens: Tokens<'a>,
    pub deadline: u64,
}

impl<'a> TradeTest<'a> {
    pub fn setup() -> TradeTest<'a> {
        let env = Env::default();
        env.mock_all_auths();

        let s_factory = create_soroswap_factory(&env);
        let s_router = create_soroswap_router(&env);
        let user = Address::generate(&env);
        let admin = Address::generate(&env);
        let adapter = create_adapter(&env);

        env.budget().reset_unlimited();
        let constellation_token = create_constellation_token(&env);
        let registry = create_registry(&env);
        let router = create_router(&env);
        let factory = create_factory(&env);
        let constellation_token_bytes = upload_constellation_token(&env);
        let pair = create_soroswap_pair_contract(&env);

        let mut tokens: Tokens = (
            create_token_contract(&env, &admin),
            create_token_contract(&env, &admin),
            create_token_contract(&env, &admin),
            create_token_contract(&env, &admin),
        );

        let mint_amount = 900_000_000_000_0000000;
        tokens.0.mint(&user, &mint_amount);
        tokens.1.mint(&user, &mint_amount);
        tokens.2.mint(&user, &mint_amount);
        tokens.3.mint(&user, &mint_amount);

        let pair_wasm = pair_contract_wasm(&env);
        s_factory.initialize(&admin, &pair_wasm);
        s_router.initialize(&s_factory.address);
        adapter.initialize(&s_router.address, &s_factory.address);

        registry.initialize(&admin);

        factory.initialize(&admin, &constellation_token_bytes);

        router.initialize(&factory.address, &s_router.address);

        let amount_0: i128 = 10_000_000_000_0000000;
        let amount_1: i128 = 10_000_000_000_0000000;

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.0.address,
            &tokens.1.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.2.address,
            &tokens.3.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.0.address,
            &tokens.2.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.0.address,
            &tokens.3.address,
            &amount_0,
            &amount_1,
        );

        Self {
            env,
            user,
            admin,
            adapter,
            s_router,
            s_factory,
            router,
            registry,
            factory,
            constellation_token,
            tokens,
            deadline: 10000000u64,
        }
    }
}
