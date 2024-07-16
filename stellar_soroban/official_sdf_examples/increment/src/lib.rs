#![no_std]
use soroban_sdk::{Symbol, symbol_short, contract, contractimpl, Env, log, auth::Context, IntoVal, unwrap::UnwrapOptimized};

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
        log!(&env, "count: {}", count);
        count = count + 1;
        let mut METHOD_CALL_EXPRESSION_30 = env.storage();
        let mut METHOD_CALL_EXPRESSION_29 = METHOD_CALL_EXPRESSION_30.instance();
        METHOD_CALL_EXPRESSION_29.set(&COUNTER, &count);
        let mut METHOD_CALL_EXPRESSION_41 = env.storage();
        let mut METHOD_CALL_EXPRESSION_40 = METHOD_CALL_EXPRESSION_41.instance();
        METHOD_CALL_EXPRESSION_40.extend_ttl(50, 100);
        return count;
    }
}


mod test;
