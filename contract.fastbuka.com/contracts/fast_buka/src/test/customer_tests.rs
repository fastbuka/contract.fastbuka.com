#![cfg(test)]
use crate::{
    FastBukaContract,
    FastBukaContractClient,
    datatypes::OrderStatus
};
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use super::common::{create_token};
extern crate std;


#[test]
fn test_customer_complete_order() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
   
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let rider = Address::generate(&env);
    
    // Create and setup token
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;

    // Mint tokens to user and record initial balances
    usdc_token.mint(&user, &total_amount);
    let initial_vendor_balance = usdc_token.balance(&vendor);
    let initial_rider_balance = usdc_token.balance(&rider);
    
    // Create order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    
    // Get order through stages
    client.update_order_status(&order_id, &vendor);
    client.pickup_order(&order_id, &rider);
    client.rider_confirms_delivery(&order_id);
    
    // Verify order is in Delivered state
    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Delivered);
    
    // Complete order as customer
    client.user_confirms_order(&order_id, &user);

    // Verify final state
    let completed_order = client.get_order(&order_id);
    assert_eq!(completed_order.status, OrderStatus::Completed);
    
    // Verify payments
    let final_vendor_balance = usdc_token.balance(&vendor);
    let final_rider_balance = usdc_token.balance(&rider);
    
    assert_eq!(final_vendor_balance, initial_vendor_balance + (total_amount - rider_fee));
    assert_eq!(final_rider_balance, initial_rider_balance + rider_fee);
}

#[test]
fn test_check_order_status_by_user() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
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

    // Test 1: Check status order created
    let _status = client.check_order_status(&user, &order_id);
    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Waiting);

    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);
    
     // Test2: Check status vendor done with food
     let __status = client.check_order_status(&user, &order_id);
     let order = client.get_order(&order_id);
     assert_eq!(order.status, OrderStatus::ReadyForPickup);
 

    

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
    let order = client.get_order(&order_id);
    let rider_confirm_no = client.get_confirmation_number_rider(&order_id);
    std::println!("rider_confirm_no: {}", rider_confirm_no);

   
    // Test3: Check status Rider picked up food
     client.check_order_status(&user, &order_id);
    assert_eq!(order.status, OrderStatus::PickedUp);
    assert_eq!(order.rider, Some(rider));
    
    // Test rider_confirms_delivery
    client.rider_confirms_delivery(&order_id);
    let order = client.get_order(&order_id);

    // Test4: Check status Rider delivered food
    client.check_order_status(&user, &order_id);
    assert_eq!(order.status, OrderStatus::Delivered);

    // Test user confirms delivery
    client.user_confirms_order(&order_id, &user);
    let order = client.get_order(&order_id);

    // Test5: Check status customer completed food delivery
    client.check_order_status(&user, &order_id);
    assert_eq!(order.status, OrderStatus::Completed);
}

#[test]
#[should_panic]
fn test_raise_dispute_when_order_not_delivered() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
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
    let reason = String::from_str(&env, "Issues with Order, food bad");
    
    // Create order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("\nOrder created with ID: {}", order_id);

    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);

    

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
    client.get_order(&order_id);
   
    // Test user confirms delivery
    client.customer_raise_dispute(&order_id, &user, &reason);

    // Test raise dispute with wrong address
    client.customer_raise_dispute(&order_id, &vendor, &reason);
}


#[test]
fn test_get_confirmation_number_customer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);
    
    // Setup test accounts
    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    
    std::println!("Test Setup:");
    std::println!("Admin: {:?}", admin);
    std::println!("User: {:?}", user);
    std::println!("Vendor: {:?}", vendor);
    
    // Create and setup token
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;
    usdc_token.mint(&user, &total_amount);
    
    // Create order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("\nOrder created with ID: {}", order_id);

    // Update order status with vendor to ReadyForPickup
    let confirmation = client.update_order_status(&order_id, &vendor);
    std::println!("Confirmation code: {:?}", confirmation);

    // Test 2: Should succeed - correct user and order is ready
    let customer_confirmation = client.get_confirmation_number_customer(&user, &order_id);
    assert_eq!(customer_confirmation, confirmation);
}