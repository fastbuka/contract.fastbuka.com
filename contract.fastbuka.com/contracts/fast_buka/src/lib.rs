#![no_std]
use crate::datatypes::{DataKey, DisputeResolution, FastBukaError, Order, OrderCreatedEvent, OrderStatus,};
use crate::interface::{
    AdminOperations, OrderManagement, RiderOperations, UserOperations, VendorOperations,
};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol, Vec};



#[contract]
pub struct FastBukaContract;



#[contractimpl]
impl OrderManagement for FastBukaContract {
    fn get_order_count(env: &Env) -> u128 {
        env.storage().persistent().get(&DataKey::OrderCounter).unwrap_or(0)
    }

    fn create_order(
        env: Env,
        user: Address,
        token: Address,
        vendor: Address,
        total_amount: i128,
        rider_fee: i128,
    ) -> Result<u128, FastBukaError> {
        // 1. Authentication
        user.require_auth();

        // add customer  address to the datakey
        env.storage().persistent().set(&DataKey::Customer(user.clone()), &true);

        // 2. Get timestamp first
        let timestamp = env.ledger().timestamp();

        // 3. get order count and increment it
        let mut count = Self::get_order_count(&env);
        count += 1;

        // 4. create token client
        let token_client = TokenClient::new(&env, &token);

        // 5. Check user's balance token amount
        let user_balance = token_client.balance(&user);
        if user_balance < total_amount {
            return Err(FastBukaError::InsufficientBalance);
        }
        if total_amount <= 0 {
            return Err(FastBukaError::InvalidAmount);
        }

        // 6 Transfer tokens
        let transfer_result =
            token_client.transfer(&user, &env.current_contract_address(), &total_amount);
        if transfer_result != () {
            return Err(FastBukaError::DepositPaymentFailed);
        }

        // 7. create order
        let order = Order {
            id: count,
            user,
            token,
            vendor,
            amount: total_amount,
            rider_fee,
            status: OrderStatus::Waiting,
            rider: None,
            created_at: timestamp,
            confirmation_number: None,
        };

        // 8. Store order using Symbol ID
        env.storage().persistent().set(&count, &order);

        // 9. Update order count
        env.storage().persistent().set(&DataKey::OrderCounter, &count);

        // Publish event
        env.events()
            .publish((DataKey::OrderCounter, symbol_short!("new_order")), count);

        // Return numeric ID
        Ok(count)
    }

    fn get_order(env: Env, order_id: u128) -> Result<Order, FastBukaError> {
        env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)
    }

    // WRITE A TEST FOR THS AND ADD CHECKS 
    fn complete_order(env: Env, order_id: u128) -> Result<(), FastBukaError> {
        // must add:  only the customer should be able to call this function.

        // 1. Get order
        let mut order: Order = env
            .storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
    
        // 2. Get order status
        if order.status != OrderStatus::Delivered {
            return Err(FastBukaError::OrderNotDelivered);
        }
    
        // 3. get token and its client - clone the token address
        let token = order.token.clone();
        let token_client = TokenClient::new(&env, &token);
    
        //4. get rider fee
        let rider_amount = order.rider_fee;
    
        // 5. get vendor total money
        let vendor_amount = order
            .amount
            .checked_sub(rider_amount)
            .ok_or(FastBukaError::CalculationError)?;
    
        // 6. transfer money to vendor - clone the vendor address
        let transfer_result = token_client.try_transfer(
            &env.current_contract_address(), 
            &order.vendor.clone(), 
            &vendor_amount
        );
        
        if let Err(_) = transfer_result {
            return Err(FastBukaError::VendorPaymentFailed);
        }
    
        // 7. transfer money to rider - clone the rider address if present
        let rider_address = order.rider.as_ref().ok_or(FastBukaError::OrderNotFound)?;
        let transfer_result = token_client.try_transfer(
            &env.current_contract_address(), 
            rider_address, 
            &rider_amount
        );
        
        if let Err(_) = transfer_result {
            return Err(FastBukaError::RiderPaymentFailed);
        }
    
        // 8. update status
        order.status = OrderStatus::Completed;
        env.storage().persistent().set(&order_id, &order);
    
        Ok(())
    }

    fn get_all_orders(env: Env) -> Result<Vec<Order>, FastBukaError> {
        // 1. Get total number of orders from our counter
        let count = Self::get_order_count(&env);
        
        // 2. Create a new vector that will hold our orders
        // The vector needs to be initialized with the environment
        let mut orders = Vec::new(&env);
    
        // 3. Loop through all possible order IDs (from 1 to count)
        let mut current_id: u128 = 1;
        while current_id <= count {
            // 4. Try to get each order from persistent storage
            // The get<K, V> method takes two type parameters:
            // K: the key type (u128 in our case)
            // V: the value type (Order in our case)
            if let Some(order) = env.storage().persistent().get::<u128, Order>(&current_id) {
                // 5. If order exists, add it to our vector
                orders.push_back(order);
            }
            current_id += 1;
        }
    
        // 6. Check if we found any orders
        if orders.is_empty() {
            return Err(FastBukaError::OrderNotFound);
        }
    
        // 7. Return the vector of orders
        Ok(orders)
    }
}

#[contractimpl]
impl VendorOperations for FastBukaContract {

    // vendor prepare a order and send it up for picku by rider
    fn update_order_status(env: Env, order_id: u128, vendor: Address) -> Result<Option<u32>, FastBukaError> {
        let mut order: Order = env.storage().persistent().get::<u128, Order>(&order_id)
            .ok_or_else(|| { FastBukaError::OrderNotFound })?;
    
        if &vendor != &order.vendor {
            return Err(FastBukaError::UnauthorizedAccess);
        }
    
        let confirmation_number = Self::generate_confirmation_number(&env);
        order.confirmation_number = Some(confirmation_number);
        order.status = OrderStatus::ReadyForPickup;
    
        env.storage().persistent().set(&order_id, &order);
        
        Ok(order.confirmation_number)
    }
        

    // Get pending orders partaining to a specific vendor.
    fn get_vendor_pending_orders(env: Env, vendor: Address) -> Result<Vec<Order>, FastBukaError> {
        let count = Self::get_order_count(&env);
        let mut vendor_orders = Vec::new(&env);
        
        let mut current_id: u128 = 1;
        while current_id <= count {
            if let Some(order) = env.storage().persistent().get::<u128, Order>(&current_id) {
                if order.vendor == vendor {
                    vendor_orders.push_back(order);
                }
            }
            current_id += 1;
        }
        
        if vendor_orders.is_empty() {
            return Err(FastBukaError::OrderNotFound);
        }
        
        Ok(vendor_orders)
    }


    // Helper function to generate confirmation order for a customer
    fn generate_confirmation_number(env: &Env) -> u32 {
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Get a random component using timestamp
        let random_component = timestamp % 10000;
        
        // Ensure number is between 1000-9999
        let confirmation = 1000 + (random_component as u32);
        
        confirmation
    }
}

// #[contractimpl]
// impl UserOperations for FastBukaContract {
//     fn get_confirmation_number(
//         env: Env,
//         order_id: Symbol,
//     ) -> Result<Option<u32>, FastBukaError> {
//         let order: Order = env.storage().get(&order_id)
//             .ok_or(FastBukaError::OrderNotFound)?;

//         if &env.invoker() != &order.user {
//             return Err(FastBukaError::UnauthorizedAccess);
//         }

//         if order.status != OrderStatus::ReadyForPickup {
//             return Err(FastBukaError::OrderNotReady);
//         }

//         Ok(order.confirmation_number)
//     }

//     fn check_order_ready(
//         env: Env,
//         order_id: Symbol,
//     ) -> Result<bool, FastBukaError> {
//         let order: Order = env.storage().get(&order_id)
//             .ok_or(FastBukaError::OrderNotFound)?;

//         if &env.invoker() != &order.user {
//             return Err(FastBukaError::UnauthorizedAccess);
//         }

//         Ok(order.status == OrderStatus::ReadyForPickup && order.confirmation_number.is_some())
//     }
// }

// #[contractimpl]
// impl RiderOperations for FastBukaContract {
//     fn pickup_order(
//         env: Env,
//         order_id: Symbol,
//         rider: Address,
//         confirmation_number: u32,
//     ) -> Result<(), FastBukaError> {
//         let mut order: Order = env.storage().get(&order_id)
//             .ok_or(FastBukaError::OrderNotFound)?;

//         if order.status != OrderStatus::ReadyForPickup {
//             return Err(FastBukaError::OrderNotReady);
//         }

//         if order.confirmation_number != Some(confirmation_number) {
//             return Err(FastBukaError::InvalidConfirmationNumber);
//         }

//         order.rider = Some(rider.clone());
//         order.status = OrderStatus::PickedUp;
//         env.storage().set(&order_id, &order);

//         env.events().publish((
//             Symbol::new(&env, "order_picked_up"),
//             OrderPickedUpEvent {
//                 order_id,
//                 rider,
//             },
//         ));

//         Ok(())
//     }
// }

// #[contractimpl]
// impl AdminOperations for FastBukaContract {
//     fn __constructor(env: Env, admin: Address, token: Address) {
//         env.storage().set(&Symbol::new(&env, "admin"), &admin);
//         env.storage().set(&Symbol::new(&env, "token"), &token);
//     }

//     fn resolve_dispute(
//         env: Env,
//         order_id: Symbol,
//         resolution: DisputeResolution,
//     ) -> Result<(), FastBukaError> {
//         let admin = env.storage().get::<_, Address>(&Symbol::new(&env, "admin"))
//             .ok_or(FastBukaError::NotAdmin)?;
//         if &env.invoker() != &admin {
//             return Err(FastBukaError::UnauthorizedAccess);
//         }

//         let mut order: Order = env.storage().get(&order_id)
//             .ok_or(FastBukaError::OrderNotFound)?;

//         if order.status != OrderStatus::Disputed {
//             return Err(FastBukaError::InvalidStatus);
//         }

//         let token = env.storage().get::<_, Address>(&Symbol::new(&env, "token"))
//             .ok_or(FastBukaError::PaymentFailed)?;
//         let token_client = TokenClient::new(&env, &token);

//         match resolution {
//             DisputeResolution::UserFault => {
//                 let vendor_amount = (order.amount * 98) / 100;
//                 let rider_amount = (order.amount * 2) / 100;

//                 token_client.transfer(
//                     &env.current_contract_address(),
//                     &order.vendor,
//                     &vendor_amount,
//                 )?;

//                 if let Some(rider) = &order.rider {
//                     token_client.transfer(
//                         &env.current_contract_address(),
//                         rider,
//                         &rider_amount,
//                     )?;
//                 }
//             },
//             DisputeResolution::VendorFault => {
//                 let refund_amount = (order.amount * 98) / 100;
//                 token_client.transfer(
//                     &env.current_contract_address(),
//                     &order.user,
//                     &refund_amount,
//                 )?;

//                 if let Some(rider) = &order.rider {
//                     let rider_amount = (order.amount * 2) / 100;
//                     token_client.transfer(
//                         &env.current_contract_address(),
//                         rider,
//                         &rider_amount,
//                     )?;
//                 }
//             },
//             DisputeResolution::RiderFault => {
//                 let vendor_amount = (order.amount * 98) / 100;
//                 token_client.transfer(
//                     &env.current_contract_address(),
//                     &order.vendor,
//                     &vendor_amount,
//                 )?;

//                 if let Some(rider) = &order.rider {
//                     token_client.transfer(
//                         rider,
//                         &order.user,
//                         &order.amount,
//                     )?;
//                 }
//             },
//         }

//         order.status = OrderStatus::Resolved;
//         env.storage().set(&order_id, &order);

//         env.events().publish((
//             Symbol::new(&env, "dispute_resolved"),
//             DisputeResolvedEvent {
//                 order_id,
//                 resolution,
//                 admin,
//             },
//         ));

//         Ok(())
//     }
// }

mod datatypes;
mod interface;
mod test;
mod token;
