#![no_std]
use soroban_sdk::{contract, contractimpl, Env, vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct AllocContract;

#[contractimpl]
impl AllocContract {
    pub fn sum(_env: Env, count: i128) -> i128 {
        let mut Thing_to_return: i128;
        let mut v1 = vec![];
        let mut METHOD_CALL_EXPRESSION_7 = 0..count;
        let mut METHOD_CALL_ARG_1_1 = v1.push(i);
        METHOD_CALL_EXPRESSION_7.for_each(METHOD_CALL_ARG_1_1);
        let mut sum = 0;
        let mut ITERATOR_15 = v1;
        let mut OPTION_i = v1.next();
        while let Some(i) = OPTION_i {
            sum = sum + i;
            OPTION_i = ITERATOR_15.next();
        }
        return sum;
    }
}


mod test;
