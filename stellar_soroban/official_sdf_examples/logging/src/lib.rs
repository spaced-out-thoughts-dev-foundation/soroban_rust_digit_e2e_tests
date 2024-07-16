#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, value: String)  {
        log!(&env, "Hello {}", value);
    }
}


mod test;
