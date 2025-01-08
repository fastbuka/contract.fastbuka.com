use crate::{
    datatypes::{DataKey, FastBukaError, Order, OrderStatus},
    interface::AdminOperations,
    FastBukaContract,
};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol, Vec};




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