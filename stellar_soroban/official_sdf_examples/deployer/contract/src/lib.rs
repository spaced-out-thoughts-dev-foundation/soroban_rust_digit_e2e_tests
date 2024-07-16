#![no_std]
use soroban_sdk::{Symbol, symbol_short, contract, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

const KEY: Symbol = symbol_short!("value");
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn init(env: Env, value: i128)  {
        let mut METHOD_CALL_EXPRESSION_9 = env.storage();
        let mut METHOD_CALL_EXPRESSION_8 = METHOD_CALL_EXPRESSION_9.instance();
        METHOD_CALL_EXPRESSION_8.set(&KEY, &value);
    }


    pub fn value(env: Env) -> i128 {
        let mut Thing_to_return: i128;
        let mut METHOD_CALL_EXPRESSION_21 = env.storage();
        let mut METHOD_CALL_EXPRESSION_20 = METHOD_CALL_EXPRESSION_21.instance();
        let mut METHOD_CALL_EXPRESSION_15 = METHOD_CALL_EXPRESSION_20.get(&KEY);
        Thing_to_return = METHOD_CALL_EXPRESSION_15.unwrap();
        return Thing_to_return;
    }
}
