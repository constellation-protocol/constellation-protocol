use super::setup::TradeTest;
use soroban_sdk::{testutils::Address as _, vec, Address, BytesN, Env, IntoVal, String, Vec};

#[test]
fn test_trade() {
    let test = TradeTest::setup();
    test.env.mock_all_auths();
    // units
    let units = vec![&test.env, 1000, 1000];
    // components
    let components: Vec<Address> = vec![
        &test.env,
        test.tokens.0.address.clone(),
        test.tokens.1.address.clone(),
    ];
    let name: String = "c_token".into_val(&test.env);
    let symbol: String = "token_symbol".into_val(&test.env);
    let manager = Address::generate(&test.env);

    test.constellation_token.initialize(
        &6u32,
        &components,
        &units,
        &name,
        &symbol,
        &test.admin,
        &manager,
    );
    test.tokens.0.approve(
        &test.user,
        &test.constellation_token.address,
        &10_000_000i128,
        &1000u32,
    );
    test.tokens.1.approve(
        &test.user,
        &test.constellation_token.address,
        &10_000_000i128,
        &1000u32,
    );
    test.constellation_token.mint(&test.user, &10i128);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let expiration_ledger = 1000u32;

    test.registry.add_module(&test.trade_module.address);
    test.registry.add_adapter(
        &test.trade_module.address,
        &test.router.address,
        &test.adapter.address,
    );
    test.constellation_token
        .set_registry(&test.registry.address);
    test.constellation_token
        .add_module(&test.trade_module.address);

    assert_eq!(test.constellation_token.balance(&test.user), 10);
    let balance_before_trade_token_0 = test.tokens.0.balance(&test.constellation_token.address);
    let balance_before_trade_token_1 = test.tokens.1.balance(&test.constellation_token.address);
    let balance_before_trade_token_2 = test.tokens.2.balance(&test.constellation_token.address);

    assert_eq!(balance_before_trade_token_0, 10000);
    assert_eq!(balance_before_trade_token_1, 10000);

    let path = &vec![
        &test.env,
        test.tokens.0.address.clone(),
        test.tokens.2.address.clone(),
    ];
    let amount_in = 5000i128;
    let res = test.router.router_get_amounts_out(&amount_in, path);
    let amount_out = res.get(1).unwrap();
    assert_eq!(balance_before_trade_token_2, 0);

    let c = test.constellation_token.get_components();
    assert_eq!(c.get(0).unwrap().address, test.tokens.0.address);
    assert_eq!(c.get(1).unwrap().address, test.tokens.1.address);

    test.trade_module.trade(
        &test.constellation_token.address,
        &test.router.address,
        &test.tokens.0.address,
        &test.tokens.2.address,
        &amount_in,
        &amount_out,
        &deadline,
        &expiration_ledger,
    );
    assert_eq!(
        test.tokens.2.balance(&test.constellation_token.address),
        amount_out
    );
   // check units 
    assert_eq!(
        test.tokens.2.balance(&test.constellation_token.address),
        4984
    );
    let c = test.constellation_token.get_components();
    assert_eq!(c.len(), 3);
    assert_eq!(
        test.constellation_token
            .get_component(&test.tokens.1.address)
            .unwrap()
            .unit,
        1000
    );
    assert_eq!(
        test.constellation_token
            .get_component(&test.tokens.0.address)
            .unwrap()
            .unit,
        500
    );
    assert_eq!(
        test.constellation_token
            .get_component(&test.tokens.2.address)
            .unwrap()
            .unit,
        498
    );
}
