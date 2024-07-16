#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract: Address, x: i128, y: i128) -> i128 {
        let mut Thing_to_return: i128;
        let mut client = ContractAClient::new(&env, &contract);
        Thing_to_return = client.add(&x, &y);
        return Thing_to_return;
    }
}
