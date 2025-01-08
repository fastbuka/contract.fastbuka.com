#![cfg(test)]

use crate::{
    FastBukaContract,
    FastBukaContractClient,
    datatypes::OrderStatus
};
use soroban_sdk::{testutils::Address as _, Address, Env};
use super::common::{create_token};
extern crate std;


#[test]
fn test_get_vendor_pending_orders() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let vendor1 = Address::generate(&env);
    let vendor2 = Address::generate(&env);

    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();

    usdc_token.mint(&user1, &2000);

    // Create orders for vendor1
    let _order1_id = client.create_order(&user1, &token_address, &vendor1, &500, &50);
    let _order2_id = client.create_order(&user1, &token_address, &vendor1, &300, &30);
    let __order2_id = client.create_order(&user1, &token_address, &vendor1, &300, &30);

    let _order3_id = client.create_order(&user1, &token_address, &vendor2, &300, &30);


    // Get vendor1's pending orders
    let vendor_orders = client.get_vendor_pending_orders(&vendor1);
    assert_eq!(vendor_orders.len(), 3, "Expected 2 orders for vendor1");

    let vendor_orders = client.get_vendor_pending_orders(&vendor2);
    assert_eq!(vendor_orders.len(), 1, "Expected 2 orders for vendor2");

}

#[test]
#[should_panic]
fn test_get_vendor_no_found_orders() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let vendor1 = Address::generate(&env);
    let vendor2 = Address::generate(&env);

    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();

    usdc_token.mint(&user1, &2000);

    // Create orders for vendor1
    let _order1_id = client.create_order(&user1, &token_address, &vendor1, &500, &50);
    

    // Get vendor1's pending orders
    let _vendor_orders = client.get_vendor_pending_orders(&vendor2);
}

#[test]
fn test_update_order_status_by_vendor() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    
    // Setup token and amounts
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    
    // Mint tokens for testing
    usdc_token.mint(&user, &total_amount);
    
    std::println!("\nTest Setup:");
    std::println!("Vendor: {:?}", vendor);
    
    // Create an order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("Created order with ID: {}", order_id);
    
    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);
    let updated_order = client.get_order(&order_id);
    
    // Verify status updated and confirmation number generated
    assert_eq!(updated_order.status, OrderStatus::ReadyForPickup);
    assert!(updated_order.confirmation_number.is_some());
}

#[test]
#[should_panic]
fn test_update_order_status_by_wrong_vendor() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let wrong_vendor = Address::generate(&env);
    
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    
    usdc_token.mint(&user, &total_amount);
    
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    
    // This should panic when wrong vendor tries to update
    client.update_order_status(&order_id, &wrong_vendor);
}
