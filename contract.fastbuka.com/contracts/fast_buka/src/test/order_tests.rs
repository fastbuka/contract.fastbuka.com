#![cfg(test)]
use crate::{
    FastBukaContract,
    FastBukaContractClient
};
use soroban_sdk::{testutils::Address as _, Address, Env};
use super::common::{create_token};
extern crate std;


#[test]
fn test_create_order_and_get_order() {
   let env = Env::default();
   env.mock_all_auths();
   let contract_id = env.register(FastBukaContract, ());
   let client = FastBukaContractClient::new(&env, &contract_id);

   let admin = Address::generate(&env);
   let user = Address::generate(&env);
   let vendor = Address::generate(&env);
   let total_amount: i128 = 1000;
   let rider_fee: i128 = 100;

   std::println!("Test Setup:");
   std::println!("Admin: {:?}", admin);
   std::println!("User: {:?}", user); 
   std::println!("Vendor: {:?}", vendor);

   let usdc_token = create_token(&env, &admin);
   let token_address = usdc_token.address.clone();
   usdc_token.mint(&user, &total_amount);

   std::println!("\nBalances before order:");
   std::println!("User balance: {}", usdc_token.balance(&user));
   std::println!("Contract balance: {}", usdc_token.balance(&contract_id));

   let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
   std::println!("\nOrder created with ID: {}", order_id);

   let order_count = client.get_order_count();
   std::println!("Total orders: {}", order_count);

   let order = client.get_order(&order_id);
   std::println!("\nRetrieved order details:");
   std::println!("Order ID: {}", order.id);
   std::println!("Order amount: {}", order.amount);
   std::println!("Order status: {:?}", order.status);
   std::println!("Contract balance1: {}", usdc_token.balance(&contract_id));

   assert_eq!(order.id, order_id);
   assert_eq!(order.amount, total_amount);
}


#[test]
fn test_get_all_orders() {
   let env = Env::default();
   env.mock_all_auths();
   let contract_id = env.register(FastBukaContract, ());
   let client = FastBukaContractClient::new(&env, &contract_id);

   let admin = Address::generate(&env);
   let user1 = Address::generate(&env);
   let user2 = Address::generate(&env);
   let vendor = Address::generate(&env);

   std::println!("\nCreating test orders:");
   std::println!("User1: {:?}", user1);
   std::println!("User2: {:?}", user2);

   let usdc_token = create_token(&env, &admin);
   let token_address = usdc_token.address.clone();

   let amount1: i128 = 1000;
   let amount2: i128 = 2000;
   usdc_token.mint(&user1, &amount1);
   usdc_token.mint(&user2, &amount2);

   let order1_id = client.create_order(&user1, &token_address, &vendor, &amount1, &100);
   let order2_id = client.create_order(&user2, &token_address, &vendor, &amount2, &200);
   std::println!("Created orders: {} and {}", order1_id, order2_id);

   let orders = client.get_all_orders();
   std::println!("\nRetrieved {} orders", orders.len());
   for (i, order) in orders.iter().enumerate() {
       std::println!("Order {}: Amount = {}", i+1, order.amount);
   }

   std::println!("Contract balance2: {}", usdc_token.balance(&contract_id));

   assert_eq!(orders.len(), 2);
   assert_eq!(orders.get(0).unwrap().amount, amount1);
   assert_eq!(orders.get(1).unwrap().amount, amount2);
}

