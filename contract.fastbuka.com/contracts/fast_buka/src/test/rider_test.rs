#![cfg(test)]
use crate::{
    datatypes::OrderStatus,
    FastBukaContract,
    FastBukaContractClient
};
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use super::common::create_token;
extern crate std;

#[test]
fn test_rider_pickup_flow() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let rider = Address::generate(&env);
    
    std::println!("Test Setup:");
    std::println!("Admin: {:?}", admin);
    std::println!("User: {:?}", user);
    std::println!("Vendor: {:?}", vendor);
    std::println!("Rider: {:?}", rider);
    
    // Create and setup token
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    usdc_token.mint(&user, &total_amount);
    
    // Create order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("\nOrder created with ID: {}", order_id);

    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);

    

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
    let order = client.get_order(&order_id);
    let rider_confirm_no = client.get_confirmation_number_rider(&order_id);
    std::println!("rider_confirm_no: {}", rider_confirm_no);

    assert_eq!(order.status, OrderStatus::PickedUp);
    assert_eq!(order.rider, Some(rider));
    
    // Test rider_confirms_delivery
    client.rider_confirms_delivery(&order_id);
    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Delivered);
}

#[test]
fn test_rider_dispute() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let rider = Address::generate(&env);
    let reason = String::from_str(&env, "Order details incorrect");
    

    // Create and setup token
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    usdc_token.mint(&user, &total_amount);

    
    // Create and process order through delivery
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
    let order = client.get_order(&order_id);
    let rider_confirm_no = client.get_confirmation_number_rider(&order_id);
    std::println!("rider_confirm_no: {}", rider_confirm_no);

    assert_eq!(order.status, OrderStatus::PickedUp);
    assert_eq!(order.rider, Some(rider.clone()));
    
    // Test rider_confirms_delivery
    client.rider_confirms_delivery(&order_id);
    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Delivered);

    client.rider_raise_dispute(&order_id, &rider, &reason);
    
    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Disputed);
}

#[test]
#[should_panic]
fn test_pickup_order_not_ready() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let rider = Address::generate(&env);
    let reason = String::from_str(&env, "Order details incorrect");
    

    // Create and setup token
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    usdc_token.mint(&user, &total_amount);

    
    // Create and process order through delivery
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
}

#[test]
#[should_panic]
fn test_dispute_before_delivery() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(FastBukaContract, ());
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let rider = Address::generate(&env);
    let reason = String::from_str(&env, "Order details incorrect");
    

    // Create and setup token
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    usdc_token.mint(&user, &total_amount);

    
    // Create and process order through delivery
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
    let order = client.get_order(&order_id);
    let rider_confirm_no = client.get_confirmation_number_rider(&order_id);
    std::println!("rider_confirm_no: {}", rider_confirm_no);

    assert_eq!(order.status, OrderStatus::PickedUp);
    assert_eq!(order.rider, Some(rider.clone()));
    

    client.rider_raise_dispute(&order_id, &rider, &reason);
}


