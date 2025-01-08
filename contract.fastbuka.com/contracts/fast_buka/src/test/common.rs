#![cfg(test)]

use soroban_sdk::{
    Address, 
    Env, 
    IntoVal,
};
use crate::token::token::{Token, TokenClient};

// Helper function to create test token
pub fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register(Token {}, ()));
    token.initialize(admin, &7, &"USDC".into_val(e), &"USDC".into_val(e));
    token
}