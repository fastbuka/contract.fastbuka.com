
#![cfg(test)]

use crate::{datatypes::Order,FastBukaContract, FastBukaContractClient};
use soroban_sdk::{vec, Env, String, Address, IntoVal};
use crate::token::token::{Token, TokenClient};


fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register(Token {}, ()));
    token.initialize(admin, &7, &"USDC".into_val(e), &"USDC".into_val(e));
    token
}

#[test]
fn test_create_order() {
    let env = Env::default();
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);

    // Prepare test data
    let user = Address::from_str(&env, "user_address");
    let token = Address::from_str(&env, "token_address");
    let vendor = Address::from_str(&env, "vendor_address");
    let total_amount: i128 = 1000; // Example total amount
    let rider_fee: i128 = 100;    // Example rider fee

    // Mock user's token balance
    let token_client = soroban_sdk::token::Client::new(&env, &token);
    token_client.

    // Call create_order
    let order_id = client
        .create_order(&env, &user, &token, &vendor, &total_amount, &rider_fee)
        .expect("Order creation should succeed");

    // Verify order count increment
    let order_count = client.get_order_count(&env);
    assert_eq!(order_count, 1);

    // Verify order data
    let stored_order: Order = env
        .storage()
        .persistent()
        .get(&order_id)
        .expect("Order should exist in storage");

    assert_eq!(stored_order.id, order_id);
    assert_eq!(stored_order.user, user);
    assert_eq!(stored_order.token, token);
    assert_eq!(stored_order.vendor, vendor);
    assert_eq!(stored_order.amount, total_amount);
    assert_eq!(stored_order.rider_fee, rider_fee);
    assert_eq!(stored_order.status, crate::datatypes::OrderStatus::Waiting);
}



