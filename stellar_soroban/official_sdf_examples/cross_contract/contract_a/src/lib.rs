#![no_std]
use soroban_sdk::{contract, contractimpl, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn add(x: i128, y: i128) -> i128 {
        let mut Thing_to_return: i128;
        let mut METHOD_CALL_EXPRESSION_2 = x.checked_add(y);
        Thing_to_return = METHOD_CALL_EXPRESSION_2.expect("no overflow");
        return Thing_to_return;
    }
}
