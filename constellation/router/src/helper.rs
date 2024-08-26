use crate::{auth::*, token::invoke};
use crate::error::Error;
use crate::require::require_exchange_router;
use crate::soroswap_router;
use crate::soroswap_router::router_get_amounts_in;
use crate::token::Component;
use soroban_sdk::{token, vec, Address, Env, Val, Vec};
pub fn get_required_amount_token_in(
    e: &Env,
    token_in_id: &Address,
    amount_constellation: i128,
    components: &Vec<Component>,
) -> Result<(i128, Vec<i128>), Error> {
    let router_id = require_exchange_router(&e);
    let mut total_token_in_amount = 0;
    let mut token_amounts_in: Vec<i128> = vec![e]; // amounts of each component token required

    for Component { unit, address } in components.iter() {
        let component_in_amount = amount_constellation * unit;
        let path = vec![
            e,
            token_in_id.clone(),
            address.clone(), /* component token address */
        ];

        let amounts_in = router_get_amounts_in(e, component_in_amount, &router_id, &path);

        match amounts_in.get(0) {
            Some(amount) => {
                total_token_in_amount += amount;
                token_amounts_in.push_back(amount);
            }
            None => {
                return Err(Error::AmountsInError);
            }
        }
    }

    Ok((total_token_in_amount, token_amounts_in))
}
// pub fn swap_tokens_for_exact_tokens(
//     e: &Env,
//     router_id: &Address,
//     token_amounts_in: &Vec<i128>,
//     amount_in_max: i128,
//     token_id: &Address,
//     to: &Address,
//     deadline: u64,
//     components: &Vec<Component>,
// ) -> Result<i128, Error> {
//     let mut total_spent = 0;
//     for (i, c) in components.iter().enumerate() {
//         match token_amounts_in.get(i as u32) {
//             Some(amount_out) => {
//                 let amount_in_spent = soroswap_router::swap_tokens_for_exact_tokens(
//                     e,
//                     router_id,
//                     amount_out,
//                     amount_in_max,
//                     token_id,
//                     &c.address,
//                     to,
//                     deadline,
//                 )?;
//                 total_spent += amount_in_spent;
//             }
//             None => {
//                 return Err(Error::AmountsInError);
//             }
//         }
//     }
//     Ok(total_spent)
// }
pub fn swap_tokens_for_exact_tokens(
    e: &Env,
    router_id: &Address,
    token_amounts_in: &Vec<i128>,
    amount_in: i128,
    token_id: &Address,
    to: &Address,
    constellation_token_id: &Address,
    deadline: u64,
    components: &Vec<Component>,
) -> Result<i128, Error> {
    let mut total_spent = 0;

    let c = components.get(0).unwrap();
    let amount_out = token_amounts_in.get(0).unwrap();

   // assert_eq!(c.address, token_id.clone());

    let call_data = get_swap_call_data(
           
        e,
        token_id.clone(),
        c.address.clone(),
        amount_in,
        amount_out, 
       to.clone(), 
        deadline,
    ); 
    let (function, args) = call_data.clone();
    let auth_entries = create_sub_auth(
        e,
        amount_in,
        token_id.clone(),
        c.address.clone(), 
        to.clone(),
    );

//    invoke(e, constellation_token_id, router_id, &call_data, &auth_entries);
   
   e.authorize_as_current_contract(auth_entries);

   e.invoke_contract::<Val>(router_id, &function, args);
    
    // for (i, c) in components.iter().enumerate() {
    //     // let token_client = token::Client::new(&e, &c.address);

    //     // let initial_balance = token_client.balance(&e.current_contract_address());

    //     match token_amounts_in.get(i as u32) {
    //         Some(amount_out) => {
    //             let call_data = get_swap_call_data(
           
    //                 e,
    //                 token_id.clone(),
    //                 c.address.clone(),
    //                 amount_in,
    //                 amount_out, 
    //                to.clone(), 
    //                 deadline,
    //             ); 
    //             let (function, args) = call_data.clone();
    //             let auth_entries = create_sub_auth(
    //                 e,
    //                 amount_in,
    //                 token_id.clone(),
    //                 c.address.clone(), 
    //                 to.clone(),
    //             );

    //         //    invoke(e, constellation_token_id, router_id, &call_data, &auth_entries);
               
    //            e.authorize_as_current_contract(auth_entries);

    //            e.invoke_contract::<Val>(router_id, &function, args);

    //             // let amount_in_spent = soroswap_router::swap_tokens_for_exact_tokens(
    //             //     e,
    //             //     router_id,
    //             //     amount_out,
    //             //     amount_in_max,
    //             //     token_id,
    //             //     &c.address,
    //             //     to,
    //             //     deadline,
    //             // )?;
    //             // total_spent += amount_in_spent;
    //         }
    //         None => {
    //             return Err(Error::AmountsInError);
    //         }
    //     }

        // let current_balance = token_client.balance(&e.current_contract_address());

        // let final_balance = current_balance - initial_balance;

        // if final_balance > 0 {
        //     token_client.approve(&e.current_contract_address(),constellation_token_id , &final_balance, &1000);
        // }
   // }
    Ok(total_spent)
}

pub fn get_base_token_amount_in(
    e: &Env,
    router_id: &Address,
    amount_in: i128,
    amount_out_min: i128,
    token_in: &Address,
    token_out: &Address,
    to: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    let amount_in = match token_in != token_out {
        true => soroswap_router::swap_exact_tokens_for_tokens(
            &e,
            router_id,
            amount_in,
            amount_out_min,
            token_in,
            token_out,
            &to.clone(),
            deadline,
        )?,
        false => amount_in,
    };
    Ok(amount_in)
}

pub fn refund_unspent(
    e: &Env,
    router_id: &Address,
    amount_unspent: i128,
    amount_out_min: i128,
    xlm_id: &Address,
    token_out: &Address,
    to: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    let refund = if token_out != xlm_id {
        soroswap_router::swap_exact_tokens_for_tokens(
            &e,
            &router_id,
            amount_unspent,
            0,
            &xlm_id,
            &token_out,
            &to.clone(),
            deadline,
        )?
    } else {
        token::Client::new(&e, &xlm_id).transfer(
            &e.current_contract_address(),
            &to,
            &amount_unspent,
        );
        amount_unspent
    };

    Ok(refund)
}
