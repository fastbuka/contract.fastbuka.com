#![no_std]
use soroban_sdk::{
    contract, contractimpl, token, Address, Env,
    Symbol, TokenClient, Vec, panic_with_error, contracterror,
};

#[contract]
pub struct FastBukaContract;



// Implementations
#[contractimpl]
impl OrderManagement for FastBukaContract {
    fn create_order(
        env: Env,
        user: Address,
        vendor: Address,
        amount: i128,
    ) -> Result<Symbol, FastBukaError> {
        if amount <= 0 {
            return Err(FastBukaError::InvalidAmount);
        }

        let timestamp = env.ledger().timestamp();
        let order_id = Symbol::new(&env, &format!("ORDER_{}_{}",&timestamp, &user));

        let token = env.storage().get::<_, Address>(&Symbol::new(&env, "token"))
            .ok_or(FastBukaError::InvalidAmount)?;
        let token_client = TokenClient::new(&env, &token);

        token_client.transfer(&user, &env.current_contract_address(), &amount)
            .map_err(|_| FastBukaError::PaymentFailed)?;

        let order = Order {
            id: order_id.clone(),
            user,
            vendor,
            amount,
            status: OrderStatus::Waiting,
            rider: None,
            created_at: timestamp,
            confirmation_number: None,
        };

        env.storage().set(&order_id, &order);

        // Add to vendor's pending orders
        let pending_key = Symbol::new(&env, &format!("pending_{}", &vendor));
        let mut pending_orders: Vec<Symbol> = env.storage().get(&pending_key)
            .unwrap_or(Vec::new(&env));
        pending_orders.push_back(order_id.clone());
        env.storage().set(&pending_key, &pending_orders);

        env.events().publish((
            Symbol::new(&env, "order_created"),
            OrderCreatedEvent {
                order_id: order_id.clone(),
                user,
                vendor,
                amount,
            },
        ));

        Ok(order_id)
    }

    fn get_order(env: Env, order_id: Symbol) -> Result<Order, FastBukaError> {
        env.storage().get(&order_id).ok_or(FastBukaError::OrderNotFound)
    }

    fn complete_order(env: Env, order_id: Symbol) -> Result<(), FastBukaError> {
        let mut order: Order = env.storage().get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
        
        if order.status != OrderStatus::Delivered {
            return Err(FastBukaError::InvalidStatus);
        }

        let token = env.storage().get::<_, Address>(&Symbol::new(&env, "token"))
            .ok_or(FastBukaError::PaymentFailed)?;
        let token_client = TokenClient::new(&env, &token);

        token_client.transfer(
            &env.current_contract_address(),
            &order.vendor,
            &order.amount
        ).map_err(|_| FastBukaError::PaymentFailed)?;

        order.status = OrderStatus::Completed;
        env.storage().set(&order_id, &order);

        Ok(())
    }
}

#[contractimpl]
impl VendorOperations for FastBukaContract {
    fn update_order_status(
        env: Env,
        order_id: Symbol,
        new_status: OrderStatus,
    ) -> Result<Option<u32>, FastBukaError> {
        let mut order: Order = env.storage().get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
        
        if &env.invoker() != &order.vendor {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        let old_status = order.status.clone();
        match (order.status, new_status) {
            (OrderStatus::Waiting, OrderStatus::Accepted) => (),
            (OrderStatus::Accepted, OrderStatus::Preparing) => (),
            (OrderStatus::Preparing, OrderStatus::ReadyForPickup) => {
                let confirmation_number = FastBukaContract::generate_confirmation_number(&env);
                order.confirmation_number = Some(confirmation_number);
                
                env.events().publish((
                    Symbol::new(&env, "confirmation_generated"),
                    ConfirmationGeneratedEvent {
                        order_id: order_id.clone(),
                        vendor: order.vendor.clone(),
                    },
                ));
            },
            _ => return Err(FastBukaError::InvalidStatusTransition),
        }

        order.status = new_status.clone();
        env.storage().set(&order_id, &order);

        env.events().publish((
            Symbol::new(&env, "status_updated"),
            OrderStatusUpdatedEvent {
                order_id,
                old_status,
                new_status,
            },
        ));

        Ok(order.confirmation_number)
    }

    fn get_vendor_pending_orders(env: Env, vendor: Address) -> Vec<Symbol> {
        let pending_key = Symbol::new(&env, &format!("pending_{}", &vendor));
        env.storage().get(&pending_key).unwrap_or(Vec::new(&env))
    }
}

#[contractimpl]
impl UserOperations for FastBukaContract {
    fn get_confirmation_number(
        env: Env,
        order_id: Symbol,
    ) -> Result<Option<u32>, FastBukaError> {
        let order: Order = env.storage().get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
        
        if &env.invoker() != &order.user {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        if order.status != OrderStatus::ReadyForPickup {
            return Err(FastBukaError::OrderNotReady);
        }

        Ok(order.confirmation_number)
    }

    fn check_order_ready(
        env: Env,
        order_id: Symbol,
    ) -> Result<bool, FastBukaError> {
        let order: Order = env.storage().get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
        
        if &env.invoker() != &order.user {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        Ok(order.status == OrderStatus::ReadyForPickup && order.confirmation_number.is_some())
    }
}

#[contractimpl]
impl RiderOperations for FastBukaContract {
    fn pickup_order(
        env: Env,
        order_id: Symbol,
        rider: Address,
        confirmation_number: u32,
    ) -> Result<(), FastBukaError> {
        let mut order: Order = env.storage().get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;
        
        if order.status != OrderStatus::ReadyForPickup {
            return Err(FastBukaError::OrderNotReady);
        }

        if order.confirmation_number != Some(confirmation_number) {
            return Err(FastBukaError::InvalidConfirmationNumber);
        }

        order.rider = Some(rider.clone());
        order.status = OrderStatus::PickedUp;
        env.storage().set(&order_id, &order);

        env.events().publish((
            Symbol::new(&env, "order_picked_up"),
            OrderPickedUpEvent {
                order_id,
                rider,
            },
        ));

        Ok(())
    }
}

#[contractimpl]
impl AdminOperations for FastBukaContract {
    fn __constructor(env: Env, admin: Address, token: Address) {
        env.storage().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().set(&Symbol::new(&env, "token"), &token);
    }

    fn resolve_dispute(
        env: Env,
        order_id: Symbol,
        resolution: DisputeResolution,
    ) -> Result<(), FastBukaError> {
        let admin = env.storage().get::<_, Address>(&Symbol::new(&env, "admin"))
            .ok_or(FastBukaError::NotAdmin)?;
        if &env.invoker() != &admin {
            return Err(FastBukaError::UnauthorizedAccess);
        }

        let mut order: Order = env.storage().get(&order_id)
            .ok_or(FastBukaError::OrderNotFound)?;

        if order.status != OrderStatus::Disputed {
            return Err(FastBukaError::InvalidStatus);
        }

        let token = env.storage().get::<_, Address>(&Symbol::new(&env, "token"))
            .ok_or(FastBukaError::PaymentFailed)?;
        let token_client = TokenClient::new(&env, &token);

        match resolution {
            DisputeResolution::UserFault => {
                let vendor_amount = (order.amount * 98) / 100;
                let rider_amount = (order.amount * 2) / 100;

                token_client.transfer(
                    &env.current_contract_address(),
                    &order.vendor,
                    &vendor_amount,
                )?;

                if let Some(rider) = &order.rider {
                    token_client.transfer(
                        &env.current_contract_address(),
                        rider,
                        &rider_amount,
                    )?;
                }
            },
            DisputeResolution::VendorFault => {
                let refund_amount = (order.amount * 98) / 100;
                token_client.transfer(
                    &env.current_contract_address(),
                    &order.user,
                    &refund_amount,
                )?;

                if let Some(rider) = &order.rider {
                    let rider_amount = (order.amount * 2) / 100;
                    token_client.transfer(
                        &env.current_contract_address(),
                        rider,
                        &rider_amount,
                    )?;
                }
            },
            DisputeResolution::RiderFault => {
                let vendor_amount = (order.amount * 98) / 100;
                token_client.transfer(
                    &env.current_contract_address(),
                    &order.vendor,
                    &vendor_amount,
                )?;

                if let Some(rider) = &order.rider {
                    token_client.transfer(
                        rider,
                        &order.user,
                        &order.amount,
                    )?;
                }
            },
        }

        order.status = OrderStatus::Resolved;
        env.storage().set(&order_id, &order);

        env.events().publish((
            Symbol::new(&env, "dispute_resolved"),
            DisputeResolvedEvent {
                order_id,
                resolution,
                admin,
            },
        ));

        Ok(())
    }
}


mod datatypes;
mod interface;
