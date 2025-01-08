use crate::{
    datatypes::{DataKey, FastBukaError, Order, OrderStatus},
    interface::OrderManagement,
    FastBukaContract, FastBukaContractClient, FastBukaContractArgs
};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contractimpl, symbol_short, Address, Env, Vec};



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



    // COME BACK LATER & WRITE A TEST FOR THS AND ADD CHECKS 
    fn complete_order(env: Env, order_id: u128) -> Result<(), FastBukaError> {
        // must add:  only the customer should be able to call this function.

        // 1. Get order
        let mut order: Order = env
            .storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
    
        // 2. delivered by the rider
        if order.status != OrderStatus::Delivered {
            return Err(FastBukaError::OrderNotDelivered);
        }

        // completed by the customer
        if order.status != OrderStatus::Completed {
            return Err(FastBukaError::OrderNotCompleted);
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