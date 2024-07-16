#![no_std]
use soroban_sdk::{Symbol, symbol_short, contract, contractimpl, Env, String, auth::Context, IntoVal, unwrap::UnwrapOptimized};

const COUNTER: Symbol = symbol_short!("COUNTER");
#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> i128 {
        let mut Thing_to_return: i128;
        let mut METHOD_CALL_EXPRESSION_8 = env.storage();
        let mut METHOD_CALL_EXPRESSION_7 = METHOD_CALL_EXPRESSION_8.instance();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_7.get(&COUNTER);
        let mut count: i128 = METHOD_CALL_EXPRESSION_2.unwrap_or(0);
        count = count + 1;
        let mut METHOD_CALL_EXPRESSION_29 = env.storage();
        let mut METHOD_CALL_EXPRESSION_28 = METHOD_CALL_EXPRESSION_29.instance();
        METHOD_CALL_EXPRESSION_28.set(&COUNTER, &count);
        let mut METHOD_CALL_EXPRESSION_42 = env.events();
        let mut METHOD_CALL_ARG_1_35 = (COUNTER, String::from_str(&env, "increment"));
        METHOD_CALL_EXPRESSION_42.publish(METHOD_CALL_ARG_1_35, count);
        return count;
    }
}


mod test;
