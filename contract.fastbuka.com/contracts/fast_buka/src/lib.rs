#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Vec};
use crate::datatypes::{DataKey};

#[contract]
pub struct FastBukaContract;

#[contractimpl]
impl FastBukaContract {
    pub fn __constructor(env: Env, admin: Address) {
        let mut admins = Vec::new(&env);
        admins.push_back(admin);
        env.storage().persistent().set(&DataKey::Admin, &admins);
    }
}

pub use implementations::{
    admin::*,
    order::*,
    rider::*,
    customer::*,
    vendor::*,
};

mod datatypes;
mod interface;
mod test;
mod token;
mod implementations;