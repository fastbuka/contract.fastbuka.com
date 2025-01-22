#![cfg(test)]
use crate::{
    FastBukaContract,
    FastBukaContractClient,
    datatypes::OrderStatus,
    datatypes::DisputeResolution,
};
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use super::common::{create_token};
extern crate std;



#[test]
fn test_admin_settle_raised_dispute_by_customer() {
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
    let _reason = String::from_str(&env, "Issues with Order, food bad");
    
    // Create order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    client.get_order(&order_id);
    std::println!("\nOrder created with ID: {}", order_id);

    // Update order status with vendor
    let confirmation = client.update_order_status(&order_id, &vendor);
    client.get_order(&order_id);
    std::println!("Confirmation code: {:?}", confirmation);

    

    // Test pickup_order
    client.pickup_order(&order_id, &rider);
    client.get_order(&order_id);

    client.rider_confirms_delivery(&order_id);
    client.get_order(&order_id);

   


    let reason = String::from_str(&env, "Issues with order");
    client.customer_raise_dispute(&order_id, &user, &reason);

    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Disputed);

    client.resolve_dispute(&order_id, &DisputeResolution::VendorFault, &admin);

    let resolved_order = client.get_order(&order_id);
    assert_eq!(resolved_order.status, OrderStatus::Resolved);

}
