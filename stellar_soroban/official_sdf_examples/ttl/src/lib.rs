#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    MyKey,
}

#[contract]
pub struct TtlContract;

#[contractimpl]
impl TtlContract {
    pub fn setup(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_9 = env.storage();
        let mut METHOD_CALL_EXPRESSION_8 = METHOD_CALL_EXPRESSION_9.persistent();
        METHOD_CALL_EXPRESSION_8.set(&DataKey::MyKey, &0);
        let mut METHOD_CALL_EXPRESSION_24 = env.storage();
        let mut METHOD_CALL_EXPRESSION_23 = METHOD_CALL_EXPRESSION_24.instance();
        METHOD_CALL_EXPRESSION_23.set(&DataKey::MyKey, &1);
        let mut METHOD_CALL_EXPRESSION_39 = env.storage();
        let mut METHOD_CALL_EXPRESSION_38 = METHOD_CALL_EXPRESSION_39.temporary();
        METHOD_CALL_EXPRESSION_38.set(&DataKey::MyKey, &2);
    }


    pub fn extend_persistent(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_54 = env.storage();
        let mut METHOD_CALL_EXPRESSION_53 = METHOD_CALL_EXPRESSION_54.persistent();
        METHOD_CALL_EXPRESSION_53.extend_ttl(&DataKey::MyKey, 1000, 5000);
    }


    pub fn extend_instance(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_65 = env.storage();
        let mut METHOD_CALL_EXPRESSION_64 = METHOD_CALL_EXPRESSION_65.instance();
        METHOD_CALL_EXPRESSION_64.extend_ttl(2000, 10000);
    }


    pub fn extend_temporary(env: Env)  {
        let mut METHOD_CALL_EXPRESSION_80 = env.storage();
        let mut METHOD_CALL_EXPRESSION_79 = METHOD_CALL_EXPRESSION_80.temporary();
        METHOD_CALL_EXPRESSION_79.extend_ttl(&DataKey::MyKey, 3000, 7000);
    }
}


mod test;
