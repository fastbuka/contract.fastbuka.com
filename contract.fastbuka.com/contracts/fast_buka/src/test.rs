#![cfg(test)]

use crate::{datatypes::Order,FastBukaContract, FastBukaContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, IntoVal};
use crate::token::token::{Token, TokenClient};


// A helper function.
fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register(Token {}, ()));
    token.initialize(admin, &7, &"USDC".into_val(e), &"USDC".into_val(e));
    token
}

#[test]
fn test_create_order() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);

    // Prepare test data
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let total_amount: i128 = 1000; // Example total amount
    let rider_fee: i128 = 100;    // Example rider fee

  
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address;
    usdc_token.mint(&user, &(total_amount as i128));
    // TokenClient::new(&env, &usdc_token).mint(&user, &total_amount);

    assert_eq!(usdc_token.balance(&user), total_amount);
    assert_eq!(usdc_token.balance(&contract_id), 0);

    // Call create_order
    // env: Env, user: Address, token: Address, vendor: Address, total_amount: i128, rider_fee: i128
    let order_id = client
        .create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);

    // Verify order count increment
    // let order_count = client.get_order_count(&env);
    // assert_eq!(order_count, 1);

    // Verify order data
    // let stored_order: Order = env
    //     .storage()
    //     .persistent()
    //     .get(&order_id)
    //     .expect("Order should exist in storage");

    // assert_eq!(stored_order.id, order_id);
    // assert_eq!(stored_order.user, user);
    // assert_eq!(stored_order.token, token);
    // assert_eq!(stored_order.vendor, vendor);
    // assert_eq!(stored_order.amount, total_amount);
    // assert_eq!(stored_order.rider_fee, rider_fee);
    // assert_eq!(stored_order.status, crate::datatypes::OrderStatus::Waiting);
}



