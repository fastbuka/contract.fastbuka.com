#![no_std]
use soroban_sdk::{contract};

#[contract]
pub struct FastBukaContract;


pub use implementations::{
    // admin::*,
    order::*,
    rider::*,
    user::*,
    vendor::*,
};


mod datatypes;
mod interface;
mod test;
mod token;
mod implementations;