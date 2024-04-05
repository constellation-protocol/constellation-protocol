use super::setup::TradeTest;
use soroban_sdk::{Address, testutils::Address as _, BytesN, Vec, vec, Env, IntoVal, String};

#[test]
fn swap_tokens_for_exact_tokens_amount_in_should() {
    let test = TradeTest::setup();
    test.env.mock_all_auths();
    let units = vec![&test.env, 1000, 1000];
    let name: String = "c_token".into_val(&test.env);
    let symbol: String = "token_symbol".into_val(&test.env);
    let manager = Address::generate(&test.env);
    let components: Vec<Address> = vec![&test.env, test.tokens.0.address.clone(), test.tokens.1.address.clone()];
    test.constellation_token.initialize(&6u32, &components, &units, &name, &symbol, &test.admin, &manager);
    test.tokens.0.approve(&test.user, &test.constellation_token.address,&10_000_000i128 , &1000u32);
    test.tokens.1.approve(&test.user, &test.constellation_token.address,&10_000_000i128 , &1000u32);
    test.constellation_token.mint(&test.user, &10i128);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let expiration_ledger = 1000u32;
   
   test.registry.add_module(&test.trade_module.address);
   test.registry.add_adapter(&test.trade_module.address, &test.router.address, &test.adapter.address);
   test.constellation_token.set_registry(&test.registry.address);
   test.constellation_token.add_module(&test.trade_module.address);


    assert_eq!(test.constellation_token.balance(&test.user), 10);
    assert_eq!(test.tokens.0.balance(&test.constellation_token.address), 10000);
    assert_eq!(test.tokens.1.balance(&test.constellation_token.address), 10000);
    let path = &vec![&test.env, test.tokens.0.address.clone(), test.tokens.2.address.clone()];
    let amount_in = 5000i128;
    let res = test.router.router_get_amounts_out(&amount_in, path);   
    let amount_out = res.get(1).unwrap();
    assert_eq!(test.tokens.2.balance(&test.constellation_token.address), 0);
    test.trade_module.trade(&test.constellation_token.address, &test.router.address, &test.tokens.0.address, &test.tokens.2.address, &amount_in, &amount_out, &deadline, &expiration_ledger);
    assert_eq!(test.tokens.2.balance(&test.constellation_token.address), amount_out);
    //test.env.budget().reset_unlimited();
    //let deadline: u64 = test.env.ledger().timestamp() + 1000;

    // let mut path: Vec<Address> = Vec::new(&test.env);
    // path.push_back(test.token_0.address.clone());
    // path.push_back(test.token_1.address.clone());

    // let amount_0: i128 = 1_000_000_000;
    // let amount_1: i128 = 4_000_000_000;

    // // -->
    // let mut token_2 = soroswap::create_token_contract(&test.env, &test.admin);
    // let mut token_3 = soroswap::create_token_contract(&test.env, &test.admin);

    // if &token_3.address < &token_2.address {
    //     std::mem::swap(&mut token_2, &mut token_3);
    // }
    // token_2.mint(&test.user, &10_000_000_000_000_000_000);
    // token_3.mint(&test.user, &10_000_000_000_000_000_000);

    // add_liquidity_v2(
    //     &test,
    //     &test.token_0.address,
    //     &test.token_1.address,
    //     &amount_0,
    //     &amount_1,
    // );
    // add_liquidity_v2(
    //     &test,
    //     &token_2.address,
    //     &token_3.address,
    //     &amount_0,
    //     &amount_1,
    // );

    // add_liquidity_v2(
    //     &test,
    //     &test.token_0.address,
    //     &token_2.address,
    //     &amount_0,
    //     &amount_1,
    // );
    // add_liquidity_v2(
    //     &test,
    //     &test.token_0.address,
    //     &token_3.address,
    //     &amount_0,
    //     &amount_1,
    // );

    // add_liquidity_v2(
    //     &test,
    //     &test.token_1.address,
    //     &token_2.address,
    //     &amount_0,
    //     &amount_1,
    // );
    // add_liquidity_v2(
    //     &test,
    //     &test.token_1.address,
    //     &token_3.address,
    //     &amount_0,
    //     &amount_1,
    // );
    // // -- //

    // let expected_amount_out = 5_000_000;
    // let amount_in_should = test
    //     .contract
    //     .router_get_amounts_in(&expected_amount_out, &path)
    //     .get(0)
    //     .unwrap();

    // let amounts = test.contract.swap_tokens_for_exact_tokens(
    //     &expected_amount_out, //amount_out
    //     &(amount_in_should),  // amount_in_max
    //     &path,                // path
    //     &test.user,           // to
    //     &deadline,
    // ); // deadline

    // &test.env.mock_all_auths();
    // let components = vec![
    //     &test.env,
    //     test.token_0.address.clone(),
    //     test.token_1.address.clone(),
    // ];
    // let amounts = vec![&test.env, 1000, 1000];
    // let decimal: u32 = 6;
    // let name = "c_token".into_val(&test.env);
    // let symbol = "token_symbol".into_val(&test.env);
    // let admin = Address::generate(&test.env);
    // let manager = Address::generate(&test.env);
    // let ct: ConstellationTokenClient<'_> = create_constellation_token(&test.env);

    // ct.initialize(
    //     &decimal,
    //     &components,
    //     &amounts,
    //     &name,
    //     &symbol,
    //     &admin,
    //     &manager,
    // );

    // let allowance = 10_200_000i128;

    // let mut args: Vec<Val> = vec![&test.env];
    // args.push_back(ct.address.into_val(&test.env));
    // args.push_back(test.contract.address.into_val(&test.env));
    // args.push_back(allowance.into_val(&test.env));
    // args.push_back(10000u32.into_val(&test.env));

    // ct.invoke(
    //     &token_2.address,
    //     &test.token_0.address,
    //     &(Symbol::new(&test.env, "approve"), args),
    //     &vec![&test.env],
    // );

    // assert_eq!(
    //     test.token_0.allowance(&ct.address, &test.contract.address),
    //     allowance
    // );

    // test.token_0
    //     .approve(&test.user, &ct.address, &1_200_000i128, &1000u32);
    // test.token_1
    //     .approve(&test.user, &ct.address, &1_200_000i128, &1000u32);
    // ct.mint(&test.user, &10i128);

    // // trade
    // let path: Vec<Address> = vec![
    //     &test.env,
    //     test.token_0.address.clone(),
    //     token_2.address.clone(),
    // ];
    // let amount_in = 1000; //1_000_000i128;
    // let res = &test.contract.router_get_amounts_out(&amount_in, &path);
    // let amount_out = res.get(1).unwrap();

    // let mut args: Vec<Val> = vec![&test.env];
    // args.push_back(amount_in.into_val(&test.env));
    // args.push_back(amount_out.into_val(&test.env));
    // args.push_back(path.into_val(&test.env));
    // args.push_back(ct.address.into_val(&test.env));
    // args.push_back(deadline.into_val(&test.env));

    // let adapter = create_adapter(&test.env, &test.contract.address, &test.factory.address);
    // adapter.initialize(&test.contract.address, &test.factory.address);
    // let auth_entries = adapter.create_sub_auth(
    //     &amount_in,
    //     &test.token_0.address.clone(),
    //     &token_2.address,
    //     &ct.address,
    // );
    // let bal = token_2.balance(&ct.address);
    // assert_eq!(bal, 0);
    // ct.invoke(
    //     &token_2.address,
    //     &test.contract.address,
    //     &(Symbol::new(&test.env, "swap_exact_tokens_for_tokens"), args),
    //     &auth_entries,
    // );
    // let bal = token_2.balance(&ct.address);
    // assert_eq!(bal, amount_out);
}
