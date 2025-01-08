// use crate::{
//     datatypes::{DataKey, FastBukaError, Order, OrderStatus},
//     interface::RiderOperations,
//     FastBukaContract,
// };
// use soroban_sdk::token::Client as TokenClient;
// use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol, Vec};


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

