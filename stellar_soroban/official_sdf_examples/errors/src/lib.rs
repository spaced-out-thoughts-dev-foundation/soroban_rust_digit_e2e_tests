#![no_std]
use soroban_sdk::{contract, contracterror, Symbol, symbol_short, contractimpl, Env, log, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    LimitReached = 1,
}

const COUNTER: Symbol = symbol_short!("COUNTER");
const MAX: i128 = 5;
#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> Result<i128, Error> {
        let mut Thing_to_return: Result<i128, Error>;
        let mut METHOD_CALL_EXPRESSION_8 = env.storage();
        let mut METHOD_CALL_EXPRESSION_7 = METHOD_CALL_EXPRESSION_8.instance();
        let mut METHOD_CALL_EXPRESSION_2 = METHOD_CALL_EXPRESSION_7.get(&COUNTER);
        let mut count: i128 = METHOD_CALL_EXPRESSION_2.unwrap_or(0);
        log!(&env, "count: {}", count);
        count = count + 1;
        let CONDITIONAL_JUMP_ASSIGNMENT_21 = count <= MAX;
        if CONDITIONAL_JUMP_ASSIGNMENT_21 {
            let mut METHOD_CALL_EXPRESSION_38 = env.storage();
            let mut METHOD_CALL_EXPRESSION_37 = METHOD_CALL_EXPRESSION_38.instance();
            METHOD_CALL_EXPRESSION_37.set(&COUNTER, &count);
            Thing_to_return = Ok(count);
        }
        else {
            Thing_to_return = Err(Error::LimitReached);
        }
        return Thing_to_return;
    }
}


mod test;
