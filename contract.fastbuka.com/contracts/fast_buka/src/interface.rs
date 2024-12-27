use crate::datatypes::{FastBukaError, Order, OrderStatus, DisputeResolution};
use soroban_sdk::{
    Address, Env,
    Symbol, Vec
};


pub trait OrderManagement {
    fn get_order_count(env: &Env) -> u128; 
    fn create_order(env: Env, user: Address, token: Address, vendor: Address, total_amount: i128, rider_fee: i128) -> Result<u128, FastBukaError>;
    fn get_order(env: Env, order_id: Symbol) -> Result<Order, FastBukaError>;
    fn complete_order(env: Env, order_id: Symbol) -> Result<(), FastBukaError>;
    fn get_all_orders(env: Env) -> Result<Vec<Order>, FastBukaError>;
}

pub trait VendorOperations {
    fn update_order_status(env: Env, order_id: Symbol, new_status: OrderStatus) -> Result<Option<u32>, FastBukaError>;
    fn get_vendor_pending_orders(env: Env, vendor: Address) -> Vec<Symbol>;
}

pub trait UserOperations {
    fn get_confirmation_number(env: Env, order_id: Symbol) -> Result<Option<u32>, FastBukaError>;
    fn check_order_ready(env: Env, order_id: Symbol) -> Result<bool, FastBukaError>;
}

pub trait RiderOperations {
    fn pickup_order(env: Env, order_id: Symbol, rider: Address, confirmation_number: u32) -> Result<(), FastBukaError>;
}

pub trait AdminOperations {
    fn __constructor(env: Env, admin: Address, token: Address);
    fn resolve_dispute(env: Env, order_id: Symbol, resolution: DisputeResolution) -> Result<(), FastBukaError>;
    fn get_disputed_orders(env: Env) -> Vec<Symbol>;
    fn get_dispute_details(env: Env, order_id: Symbol) -> Result<(Address, Symbol), FastBukaError>;
}

pub trait DisputeOperations {
    fn raise_dispute(env: Env, order_id: Symbol, reason: Symbol) -> Result<(), FastBukaError>;
}
