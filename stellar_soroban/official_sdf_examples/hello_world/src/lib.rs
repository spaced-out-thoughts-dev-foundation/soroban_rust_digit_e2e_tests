#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec, vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        let mut Thing_to_return: Vec<String>;
        Thing_to_return = vec![&env, String::from_str(&env, "Hello"), to];
        return Thing_to_return;
    }
}


mod test;
