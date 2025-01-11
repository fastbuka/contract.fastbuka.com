use crate::{
    datatypes::{FastBukaError, Order, OrderStatus, DataKey},
    interface::VendorOperations,
    FastBukaContract, FastBukaContractClient, FastBukaContractArgs
};
use soroban_sdk::{contractimpl, Address, Env, Vec};




#[contractimpl]
impl VendorOperations for FastBukaContract {
    // Update order status to ReadyForPickup

    fn update_order_status(env: Env, order_id: u128, vendor: Address) -> Result<u32, FastBukaError> {
        let mut order: Order = env.storage().persistent().get::<u128, Order>(&order_id)
            .ok_or_else(|| { FastBukaError::OrderNotFound })?;
    
        if &vendor != &order.vendor {
            return Err(FastBukaError::UnauthorizedAccess);
        }
    
        let confirmation_number = Self::generate_confirmation_number(&env);
        order.confirmation_number = Some(confirmation_number);  // Wrap in Some()
        order.status = OrderStatus::ReadyForPickup;
    
        env.storage().persistent().set(&order_id, &order);
        
        Ok(confirmation_number)  // Return the raw u32
    }
        

    
    fn get_vendor_pending_orders(env: Env, vendor: Address) -> Result<Vec<Order>, FastBukaError> {
       
        let count =  env.storage().persistent().get(&DataKey::OrderCounter).unwrap_or(0);
        let mut vendor_orders = Vec::new(&env); // Initialize vector for vendor orders
    
        // Iterate through all order IDs
        for current_id in 1..=count {
            // Try to get the order from storage
            if let Some(order) = env.storage().persistent().get::<u128, Order>(&current_id) {
                // Check if the vendor matches
                if order.vendor == vendor {
                    vendor_orders.push_back(order);
                }
            }
        }
    
        // Return error if no orders are found
        if vendor_orders.is_empty() {
            return Err(FastBukaError::OrderNotFound);
        }
    
        Ok(vendor_orders)
    }


    // Helper function to generate confirmation order
    fn generate_confirmation_number(env: &Env) -> u32 {
        // Get current timestamp
        let timestamp = env.ledger().sequence();
        // std::println!("Timestamp in function: {}", timestamp);
    
        
        // Get a random component using timestamp
        let random_component = timestamp % 10000;
        
        // Ensure number is between 1000-9999
        let confirmation = 1000 + (random_component as u32); 
        confirmation
    }


    
}