use crate::datatypes::{FastBukaError, Order, OrderStatus, DisputeResolution};
use soroban_sdk::{
    Address, Env, Vec, String
};


pub trait OrderManagement {
    fn get_order_count(env: &Env) -> u128; 
    fn create_order(env: Env, user: Address, token: Address, vendor: Address, total_amount: i128, rider_fee: i128) -> Result<u128, FastBukaError>;
    fn get_order(env: Env, order_id: u128) -> Result<Order, FastBukaError>;
    fn user_confirms_order(env: Env, order_id: u128, customer: Address) -> Result<(), FastBukaError>;
    fn get_all_orders(env: Env) -> Result<Vec<Order>, FastBukaError>;
}

pub trait VendorOperations {
    fn update_order_status(env: Env, order_id: u128, vendor: Address) -> Result<u32, FastBukaError>;
    fn get_vendor_pending_orders(env: Env, vendor: Address) -> Result<Vec<Order>, FastBukaError> ;
    fn generate_confirmation_number(env: &Env) -> u32;
}

pub trait RiderOperations {
    fn pickup_order(env: Env, order_id: u128, rider: Address) -> Result<(), FastBukaError>;
    fn get_confirmation_number_rider(env: Env, order_id: u128) -> Result<u32, FastBukaError>;
    fn rider_confirms_delivery(env: Env, order_id: u128) ->Result<(), FastBukaError>;
    fn rider_raise_dispute(env: Env, order_id: u128, address: Address, reason: String) -> Result<(), FastBukaError>;
}

pub trait CustomerOperations {
    fn get_confirmation_number_customer(env: Env, customer: Address, order_id: u128) -> Result<u32, FastBukaError>;
    fn check_order_status(env: Env, customer: Address, order_id: u128) -> Result<OrderStatus, FastBukaError>;
    fn customer_raise_dispute(env: Env, order_id: u128, address: Address, reason: String) -> Result<(), FastBukaError>;
}


pub trait AdminOperations {
    fn resolve_dispute(env: Env, order_id: u128, resolution: DisputeResolution, admin: Address) -> Result<(), FastBukaError>;
    fn add_admin(env: Env, caller: Address, new_admin: Address) -> Result<(), FastBukaError>;
    fn get_admins(env: Env) -> Vec<Address>;
    fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) -> Result<(), FastBukaError>;
    fn get_all_disputed_orders(env: Env) -> Vec<u128>;
}  