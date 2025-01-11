use crate::{
    datatypes::{FastBukaError, Order, OrderStatus},
    interface::RiderOperations,
    FastBukaContract, FastBukaContractArgs, FastBukaContractClient
};
use soroban_sdk::{contractimpl, Address, Env, String, Symbol};



#[contractimpl]
impl RiderOperations for FastBukaContract {
    
    fn get_confirmation_number_rider(
        env: Env,
        order_id: u128,
    ) -> Result<u32, FastBukaError> {
        // Get order from storage
        let order: Order = env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // Check if order is in ReadyForPickup status
        if order.status != OrderStatus::PickedUp {
            return Err(FastBukaError::OrderNotPickedUp);
        }

        // Return confirmation number if it exists
        order.confirmation_number.ok_or(FastBukaError::InvalidConfirmationNumber)
    }

    
    fn pickup_order(
        env: Env,
        order_id: u128,
        rider: Address,
    ) -> Result<(), FastBukaError> {
        // Verify rider's authorization
        rider.require_auth();

        // Get order from storage
        let mut order: Order = env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // Check if order is ready for pickup
        if order.status != OrderStatus::ReadyForPickup {
            return Err(FastBukaError::OrderNotReady);
        }

        // Update order with rider and status
        order.rider = Some(rider.clone());
        order.status = OrderStatus::PickedUp;
        env.storage().persistent().set(&order_id, &order);

        // Publish pickup event
        env.events().publish(
            (Symbol::new(&env, "order_picked_up"), order_id),
            rider
        );

        Ok(())
    }

    fn rider_confirms_delivery(env: Env, order_id: u128) -> Result<(), FastBukaError> {
        // Get order from storage
        let mut order: Order = env.storage()
            .persistent()
            .get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        // Check if order is in PickedUp status
        if order.status != OrderStatus::PickedUp {
            return Err(FastBukaError::InvalidStatus);
        }
        

        // Update order status to Delivered
        order.status = OrderStatus::Delivered;
        env.storage().persistent().set(&order_id, &order);

        // Publish delivery event
        env.events().publish(
            (Symbol::new(&env, "order_delivered"), order_id),
            order.rider.unwrap()
        );

        Ok(())
    }  


      fn rider_raise_dispute(
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
        if Some(address.clone()) != order.rider {
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

        // Publish dispute event
        env.events().publish(
            (Symbol::new(&env, "dispute_raised"), order_id),
            (address, reason.clone())
        );
        Ok(())
    }

}

