use crate::{
    datatypes::{FastBukaError, Order, OrderStatus, DataKey},
    interface::CustomerOperations,
    FastBukaContract, FastBukaContractClient, FastBukaContractArgs
};

use soroban_sdk::{contractimpl, Address, Env, String, Symbol, Vec};




#[contractimpl]
impl CustomerOperations for FastBukaContract {

    fn get_confirmation_number_customer(
        env: Env,
        customer: Address,
        order_id: u128,
    ) -> Result<u32, FastBukaError> {
        // Verify customer's authorization
        customer.require_auth();

        // Get order from storage
        let order: Order = env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // Check if the caller is the order's customer
        if customer != order.user {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        // Check if order is in ReadyForPickup status
        if order.status != OrderStatus::ReadyForPickup {
            return Err(FastBukaError::OrderNotReady);
        }

        // Return confirmation number if it exists
        order.confirmation_number.ok_or(FastBukaError::InvalidConfirmationNumber)
    }


    fn check_order_status(
        env: Env,
        customer: Address,
        order_id: u128,
    ) -> Result<OrderStatus, FastBukaError> {
        // Verify customer's authorization
        customer.require_auth();

        // Get order from storage
        let order: Order = env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // Check if the caller is the order's customer
        if customer != order.user {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        // Return the current order status
        Ok(order.status)
    }


    // fn user_confirms_order(env: Env, order_id: u128, address: Address) -> Result<(), FastBukaError> {
    //     // Get order from storage
    //     let mut order: Order = env.storage()
    //         .persistent()
    //         .get(&order_id)
    //         .ok_or(FastBukaError::OrderNotFound)?;

    //     // Require authorization from the order's user
    //     address.require_auth();

    //     // Check if order is in Delivered status by Rider
    //     if order.status != OrderStatus::Delivered {
    //         return Err(FastBukaError::InvalidStatus);
    //     }

    //     // Update order status to Completed
    //     order.status = OrderStatus::Completed;
    //     env.storage().persistent().set(&order_id, &order);

    //     Ok(())
    // }


    fn customer_raise_dispute(
        env: Env,
        order_id: u128,
        address: Address,
        reason: String,
    ) -> Result<(), FastBukaError> {
        // Verify caller's authorization
        address.require_auth();

        // Get order from storage
        let mut order: Order = env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // Check if caller is the order's customer
        if address != order.user {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        // Check if order is in a valid status for dispute
        if order.status != OrderStatus::Delivered {
            return Err(FastBukaError::OrderNotDelivered);
        }

        // check if order has already been marked as completed
        if order.status == OrderStatus::Completed {
            return Err(FastBukaError::OrderNotCompleted)
        }

        // Check if dispute already exists
        if order.status == OrderStatus::Disputed {
            return Err(FastBukaError::DisputeAlreadyExists);
        }

        // Update order status to Disputed
        order.status = OrderStatus::Disputed;
        env.storage().persistent().set(&order_id, &order);

        // Add order ID to the disputed orders list
        let mut disputed_orders: Vec<u128> = env.storage().persistent()
        .get(&DataKey::DisputedOrders)
        .unwrap_or_else(|| Vec::new(&env));

        if !disputed_orders.contains(&order_id) {
            disputed_orders.push_back(order_id);
            env.storage().persistent().set(&DataKey::DisputedOrders, &disputed_orders);
        }

        // Publish dispute event
        env.events().publish(
            (Symbol::new(&env, "dispute_raised"), order_id),
            (address, reason.clone())
        );
        Ok(())
    }
}
