#![no_std]
use soroban_sdk::{contract, contracttype, Address, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Counter(Address),
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env, user: Address, value: i128) -> i128 {
        let mut Thing_to_return: i128;
        user.require_auth();
        let mut CALL_EXPRESSION_ARG_1_3 = user.clone();
        let mut key = DataKey::Counter(CALL_EXPRESSION_ARG_1_3);
        let mut METHOD_CALL_EXPRESSION_16 = env.storage();
        let mut METHOD_CALL_EXPRESSION_15 = METHOD_CALL_EXPRESSION_16.persistent();
        let mut METHOD_CALL_EXPRESSION_10 = METHOD_CALL_EXPRESSION_15.get(&key);
        let mut count: i128 = METHOD_CALL_EXPRESSION_10.unwrap_or_default();
        count = count + value;
        let mut METHOD_CALL_EXPRESSION_37 = env.storage();
        let mut METHOD_CALL_EXPRESSION_36 = METHOD_CALL_EXPRESSION_37.persistent();
        METHOD_CALL_EXPRESSION_36.set(&key, &count);
        return count;
    }
}


mod test;
