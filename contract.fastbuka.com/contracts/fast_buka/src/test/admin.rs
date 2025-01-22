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
fn test_admin_management() {
    let env = Env::default();
    env.mock_all_auths();

    // Setup initial admin
    let admin1 = Address::generate(&env);
    let contract_id = env.register(FastBukaContract, (admin1.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);

    // Verify initial admin is set
    let admins = client.get_admins();
    assert_eq!(admins.len(), 1);
    assert!(admins.contains(&admin1));

    // Test adding new admin
    let admin2 = Address::generate(&env);
    client.add_admin(&admin1, &admin2);

    // Verify second admin was added
    let admins = client.get_admins();
    assert_eq!(admins.len(), 2);
    assert!(admins.contains(&admin2));
    // Test removing admin
    client.remove_admin(&admin1, &admin2);

    // Verify admin was removed
    let admins = client.get_admins();
    assert_eq!(admins.len(), 1);
    assert!(!admins.contains(&admin2));
}


#[test]
#[should_panic]
fn test_non_admin_cannot_add_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);

    // Non-admin tries to add new admin
    client.add_admin(&non_admin, &Address::generate(&env));
}

#[test]
#[should_panic]
fn test_non_admin_cannot_remove_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);

    // Non-admin tries to remove admin
    client.remove_admin(&non_admin, &admin);
}


#[test]
fn test_get_all_disputed_orders() {
    let env = Env::default();
    env.mock_all_auths();

    // Setup contract and accounts
    let admin = Address::generate(&env);
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let vendor = Address::generate(&env);
    let rider = Address::generate(&env);

    // Create token and setup
    let usdc_token = create_token(&env, &admin);
    let token_address = usdc_token.address.clone();
    let total_amount: i128 = 1000;
    let rider_fee: i128 = 100;

    // Mint tokens to user
    usdc_token.mint(&user, &(total_amount * 2)); // Double amount for two orders
    
    // Approve with expiration ledger
    let expiration_ledger: u32 = 1000;  // Set appropriate expiration
    usdc_token.approve(&user, &contract_id, &(total_amount * 2), &expiration_ledger);

    std::println!("User balance before: {}", usdc_token.balance(&user));
    std::println!("Contract balance before: {}", usdc_token.balance(&contract_id));

    // Create first order
    let order_id1 = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("Order 1 created");
    std::println!("User balance after order 1: {}", usdc_token.balance(&user));

    // Complete order flow for first order
    client.update_order_status(&order_id1, &vendor);
    client.pickup_order(&order_id1, &rider);
    client.rider_confirms_delivery(&order_id1);
    
    // Create second order
    let order_id2 = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("Order 2 created");
    std::println!("User balance after order 2: {}", usdc_token.balance(&user));

    // Complete order flow for second order
    client.update_order_status(&order_id2, &vendor);
    client.pickup_order(&order_id2, &rider);
    client.rider_confirms_delivery(&order_id2);

    // Raise disputes
    let reason = String::from_str(&env, "Test dispute");
    client.customer_raise_dispute(&order_id1, &user, &reason.clone());
    std::println!("Dispute 1 raised");

    client.customer_raise_dispute(&order_id2, &user, &reason);
    std::println!("Dispute 2 raised");

    // Get and verify disputed orders
    let disputed_orders = client.get_all_disputed_orders();
    std::println!("Disputed orders count: {}", disputed_orders.len());
    
    assert_eq!(disputed_orders.len(), 2);
    assert!(disputed_orders.contains(&order_id1));
    assert!(disputed_orders.contains(&order_id2));
}

#[test]
#[should_panic]
fn test_cannot_add_duplicate_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(FastBukaContract, (admin.clone(),));
    let client = FastBukaContractClient::new(&env, &contract_id);

    // Try to add same admin again
    client.add_admin(&admin, &admin);
}

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

    // Mint tokens and approve spending
    usdc_token.mint(&user, &total_amount);
    let expiration_ledger: u32 = 1000;
    usdc_token.approve(&user, &contract_id, &total_amount, &expiration_ledger);

    // Check initial balances
    let initial_user_balance = usdc_token.balance(&user);
    std::println!("Initial user balance: {}", initial_user_balance);
    assert_eq!(initial_user_balance, total_amount);

    // Create order
    let order_id = client.create_order(&user, &token_address, &vendor, &total_amount, &rider_fee);
    std::println!("\nOrder created with ID: {}", order_id);

    // Check balances after order creation
    let user_balance_after_order = usdc_token.balance(&user);
    let contract_balance = usdc_token.balance(&contract_id);
    std::println!("User balance after order: {}", user_balance_after_order);
    std::println!("Contract balance after order: {}", contract_balance);
    assert_eq!(user_balance_after_order, 0); // All tokens should be in contract
    assert_eq!(contract_balance, total_amount);

    // Complete order flow
    client.update_order_status(&order_id, &vendor);
    client.pickup_order(&order_id, &rider);
    client.rider_confirms_delivery(&order_id);

    // Raise dispute
    let reason = String::from_str(&env, "Issues with order");
    client.customer_raise_dispute(&order_id, &user, &reason);

    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Disputed);

    // Resolve dispute as vendor fault
    client.resolve_dispute(&order_id, &DisputeResolution::VendorFault, &admin);

    // Check final balances
    let final_user_balance = usdc_token.balance(&user);
    let final_rider_balance = usdc_token.balance(&rider);
    let final_vendor_balance = usdc_token.balance(&vendor);
    let final_contract_balance = usdc_token.balance(&contract_id);

    std::println!("Final balances:");
    std::println!("User: {}", final_user_balance);
    std::println!("Rider: {}", final_rider_balance);
    std::println!("Vendor: {}", final_vendor_balance);
    std::println!("Contract: {}", final_contract_balance);

    // In VendorFault case:
    // - Customer gets refund minus rider fee
    // - Rider gets their fee
    // - Vendor gets nothing
    assert_eq!(final_user_balance, total_amount - rider_fee);
    assert_eq!(final_rider_balance, rider_fee);
    assert_eq!(final_vendor_balance, 0);
    assert_eq!(final_contract_balance, 0);

    let resolved_order = client.get_order(&order_id);
    assert_eq!(resolved_order.status, OrderStatus::Resolved);
}