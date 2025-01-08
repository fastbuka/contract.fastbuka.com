#![cfg(test)]
use crate::{
    FastBukaContract,
    FastBukaContractClient,
    datatypes::OrderStatus
};
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use super::common::{create_token};
extern crate std;


// #[test]
// fn test_confirm_order() {
//     let env = Env::default();
//     env.mock_all_auths();
//     let contract_id = env.register(FastBukaContract, ());
//     let client = FastBukaContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     let user = Address::generate(&env);
//     let vendor = Address::generate(&env);
    
//     let usdc_token = create_token(&env, &admin);
//     let token_address = usdc_token.address.clone();
//     let total_amount: i128 = 1000;
//     usdc_token.mint(&user, &total_amount);

//     let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &100);
    
//     // Set order to Delivered status first
//     let mut order = client.get_order(&order_id);
//     order.status = OrderStatus::Delivered;
//     env.storage().persistent().set(&order_id, &order);

//     client.user_confirms_order(&order_id);
    
//     let updated_order = client.get_order(&order_id);
//     assert_eq!(updated_order.status, OrderStatus::Completed);
// }

// #[test]
// fn test_check_order_status() {
//     let env = Env::default();
//     env.mock_all_auths();
//     let contract_id = env.register(FastBukaContract, ());
//     let client = FastBukaContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     let user = Address::generate(&env);
//     let vendor = Address::generate(&env);
    
//     let usdc_token = create_token(&env, &admin);
//     let token_address = usdc_token.address.clone();
//     usdc_token.mint(&user, &1000);

//     let order_id = client.create_order(&user, &token_address, &vendor, &500, &50);
    
//     let status = client.check_order_status(&user, &order_id);
//     assert!(status.is_ok());
// }

// #[test]
// #[should_panic]
// fn test_raise_dispute_completed_order() {
//     let env = Env::default();
//     env.mock_all_auths();
//     let contract_id = env.register(FastBukaContract, ());
//     let client = FastBukaContractClient::new(&env, &contract_id);

//     let admin = Address::generate(&env);
//     let user = Address::generate(&env);
//     let vendor = Address::generate(&env);
    
//     let usdc_token = create_token(&env, &admin);
//     let token_address = usdc_token.address.clone();
//     usdc_token.mint(&user, &1000);

//     let order_id = client.create_order(&user, &token_address, &vendor, &500, &50);
    
//     // Set order to Completed status
//     let mut order = client.get_order(&order_id);
//     order.status = OrderStatus::Completed;
//     env.storage().persistent().set(&order_id, &order);

//     // Update order status with vendor
//     let confirmation = client.update_order_status(&order_id, &vendor);
//     std::println!("Confirmation code: {:?}", confirmation);
//     let updated_order = client.get_order(&order_id);

//     let reason = String::from_slice(&env, "Order not received");
//     // Should panic
//     client.raise_dispute(&order_id, &user, &reason);
// }