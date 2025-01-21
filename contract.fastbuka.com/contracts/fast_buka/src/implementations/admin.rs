use crate::{
    datatypes::{DataKey, FastBukaError, Order, DisputeResolvedEvent, OrderStatus, DisputeResolution},
    interface::AdminOperations,
    FastBukaContract, FastBukaContractArgs, FastBukaContractClient
};
use soroban_sdk::{contractimpl, Address, Env, Vec, token::Client as TokenClient, symbol_short};


#[contractimpl]
impl AdminOperations for FastBukaContract {   

    // Getter function for admin
    fn get_admins(env: Env) -> Vec<Address> {
        env.storage().persistent()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| Vec::new(&env))
    }

    fn add_admin(env: Env, caller: Address, new_admin: Address) -> Result<(), FastBukaError> {
        // Verify the caller is an existing admin
        caller.require_auth();

        // Fetch the current list of admins from storage
        let mut admins: Vec<Address> = env.storage().persistent()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| Vec::new(&env));

        // Check if the caller is in the admin list
        if !admins.contains(&caller) {
            return Err(FastBukaError::NotAdmin);
        }

        // Add the new admin if not already in the list
        if admins.contains(&new_admin) {
            return Err(FastBukaError::AlreadyAdmin);
        }

        admins.push_back(new_admin);
        env.storage().persistent().set(&DataKey::Admin, &admins);
        Ok(())
    } 


    fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) -> Result<(), FastBukaError> {
        // Verify the caller is an existing admin
        caller.require_auth();

        // Fetch the current list of admins from storage
        let mut admins: Vec<Address> = env.storage().persistent()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| Vec::new(&env));

        // Check if the caller is in the admin list
        if !admins.contains(&caller) {
            return Err(FastBukaError::NotAdmin);
        }

        // Check if the admin to remove exists in the list
        if !admins.contains(&admin_to_remove) {
            return Err(FastBukaError::NotAdmin);
        }

       // Find the index of the admin to remove
        let remove_index = admins.iter()
        .position(|admin| admin == admin_to_remove)
        .ok_or(FastBukaError::NotAdmin)?;

        // Remove the admin at the found index
        admins.remove(remove_index.try_into().unwrap());
        env.storage().persistent().set(&DataKey::Admin, &admins);

        Ok(())
    }


    // Admin operation to retrieve all disputed orders
    fn get_all_disputed_orders(env: Env) -> Vec<u128> {
        // Retrieve all disputed order IDs
        let disputed_orders: Vec<u128> = env.storage().persistent()
            .get(&DataKey::DisputedOrders)
            .unwrap_or_else(|| Vec::new(&env));
    
        // Map the order IDs to include reasons (or other metadata)
        let mut disputes_with_details = Vec::new(&env);
        for order_id in disputed_orders.iter() {
            if let Some(_order) = env.storage().persistent().get::<u128, Order>(&order_id) {
                disputes_with_details.push_back(
                    order_id.clone(),
                );
            }
        }
    
        disputes_with_details
    }



    fn resolve_dispute(env: Env, order_id: u128, resolution: DisputeResolution, admin: Address) -> Result<(), FastBukaError> {
        // 1. Verify admin authentication
        admin.require_auth();

        // 2. Check if admin is actually an admin
        let admins = Self::get_admins(env.clone());
        if !admins.contains(&admin) {
            return Err(FastBukaError::NotAdmin);
        }

        // 3. Get the disputed order
        let mut order: Order = env
            .storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // 4. Verify order is in disputed status
        if order.status != OrderStatus::Disputed {
            return Err(FastBukaError::DisputeNotFound);
        }


        // 5. Get token client
        let token_client = TokenClient::new(&env, &order.token);
        let contract_balance = token_client.balance(&env.current_contract_address());

        if order.amount > contract_balance {
            return Err(FastBukaError::InsufficientFundsInContract);
        }

        match resolution {
            DisputeResolution::VendorFault => {
                // Get rider fee
                let rider_amount = order.rider_fee;
                
                // Calculate customer refund amount (total minus rider fee)
                let customer_amount = order.amount
                    .checked_sub(rider_amount)
                    .ok_or(FastBukaError::CalculationError)?;
    
                // Transfer to customer
                let transfer_result = token_client.try_transfer(
                    &env.current_contract_address(),
                    &order.user,
                    &customer_amount
                );

                if let Err(_) = transfer_result {
                    return Err(FastBukaError::CustomerPaymentFailed);
                }
    
                // Pay rider if exists
                if let Some(rider) = &order.rider {
                    let rider_transfer_payment = token_client.try_transfer(
                        &env.current_contract_address(),
                        rider,
                        &rider_amount
                    );
                    if let Err(_) = rider_transfer_payment {
                        return Err(FastBukaError::RiderPaymentFailed);
                    }
                }
            },
            DisputeResolution::RiderFault => {
                // Get rider fee (this goes back to customer)
                let rider_amount = order.rider_fee;
                
                // Calculate vendor amount (total minus rider fee)
                let vendor_amount = order.amount
                    .checked_sub(rider_amount)
                    .ok_or(FastBukaError::CalculationError)?;
    
                // Transfer rider fee back to customer
                let transfer_payment = token_client.try_transfer(
                    &env.current_contract_address(),
                    &order.user,
                    &rider_amount
                );
                if let Err(_) = transfer_payment {
                    return Err(FastBukaError::CustomerPaymentFailed);
                }
                
    
                // Pay vendor
                let vendor_payment = token_client.try_transfer(
                    &env.current_contract_address(),
                    &order.vendor,
                    &vendor_amount
                );
                if let Err(_) = vendor_payment {
                    return Err(FastBukaError::VendorPaymentFailed);
                }
            },
            DisputeResolution::CustomerFault => {
                // Get rider fee
                let rider_amount = order.rider_fee;
                
                // Calculate vendor amount (total minus rider fee)
                let vendor_amount = order.amount
                    .checked_sub(rider_amount)
                    .ok_or(FastBukaError::CalculationError)?;
    
                // Pay vendor
                let vendor_payment = token_client.try_transfer(
                    &env.current_contract_address(),
                    &order.vendor,
                    &vendor_amount
                );
                if let Err(_) = vendor_payment {
                    return Err(FastBukaError::VendorPaymentFailed);
                }
    
                // Pay rider if exists
                if let Some(rider) = &order.rider {
                    let rider_payment = token_client.try_transfer(
                        &env.current_contract_address(),
                        rider,
                        &rider_amount
                    );
                    if let Err(_) = rider_payment {
                        return Err(FastBukaError::RiderPaymentFailed);
                    }
                }
            }
        }

        // 7. Update order status to resolved
        order.status = OrderStatus::Resolved;
        env.storage().persistent().set(&order_id, &order);

        // 8. Remove from disputed orders list
        let mut disputed_orders: Vec<u128> = env
        .storage()
        .persistent()
        .get(&DataKey::DisputedOrders)
        .unwrap_or_else(|| Vec::new(&env));

        if let Some(index) = disputed_orders.iter().position(|id| id.clone() == order_id) {
            disputed_orders.remove(index.try_into().unwrap());
            env.storage().persistent().set(&DataKey::DisputedOrders, &disputed_orders);
        }

        // 9. Emit dispute resolved event
        env.events().publish(
            (symbol_short!("dispute"), symbol_short!("resolved")),
            DisputeResolvedEvent {
                order_id,
                resolution,
                admin
            }
        );
        Ok(())
    }
    
   
}