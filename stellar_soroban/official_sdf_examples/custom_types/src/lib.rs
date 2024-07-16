#![no_std]
use soroban_sdk::{contract, contracttype, Symbol, symbol_short, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub count: i128,
    pub last_incr: i128,
}

const STATE: Symbol = symbol_short!("STATE");
#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, incr: i128) -> i128 {
        let mut Thing_to_return: i128;
        let mut CALL_EXPRESSION_ARG_1_0 = env.clone();
        let mut state = Self::get_state(CALL_EXPRESSION_ARG_1_0);
        state.count = state.count + incr;
        state.last_incr = incr;
        let mut METHOD_CALL_EXPRESSION_32 = env.storage();
        let mut METHOD_CALL_EXPRESSION_31 = METHOD_CALL_EXPRESSION_32.instance();
        METHOD_CALL_EXPRESSION_31.set(&STATE, &state);
        return state.count;
    }


    pub fn get_state(env: Env) -> State {
        let mut Thing_to_return: State;
        let mut METHOD_CALL_EXPRESSION_53 = env.storage();
        let mut METHOD_CALL_EXPRESSION_52 = METHOD_CALL_EXPRESSION_53.instance();
        let mut METHOD_CALL_EXPRESSION_47 = METHOD_CALL_EXPRESSION_52.get(&STATE);
        let mut METHOD_CALL_ARG_1_42 = State{count: 0, last_incr: 0};
        Thing_to_return = METHOD_CALL_EXPRESSION_47.unwrap_or(METHOD_CALL_ARG_1_42);
        return Thing_to_return;
    }
}


mod test;
