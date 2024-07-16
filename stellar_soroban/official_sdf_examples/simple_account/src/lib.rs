#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, BytesN, Vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Owner,
}

#[contract]
pub struct SimpleAccount;

#[contractimpl]
impl SimpleAccount {
    pub fn init(env: Env, public_key: BytesN<32>)  {
        let mut METHOD_CALL_EXPRESSION_7 = env.storage();
        let mut METHOD_CALL_EXPRESSION_6 = METHOD_CALL_EXPRESSION_7.instance();
        let mut CONDITIONAL_JUMP_ASSIGNMENT_1 = METHOD_CALL_EXPRESSION_6.has(&DataKey::Owner);
        if CONDITIONAL_JUMP_ASSIGNMENT_1 {
            panic!("owner is already set");
        }
        let mut METHOD_CALL_EXPRESSION_26 = env.storage();
        let mut METHOD_CALL_EXPRESSION_25 = METHOD_CALL_EXPRESSION_26.instance();
        METHOD_CALL_EXPRESSION_25.set(&DataKey::Owner, &public_key);
    }


    pub fn __check_auth(env: Env, signature_payload: BytesN<32>, signature: BytesN<64>, _auth_context: Vec<Context>)  {
        let mut METHOD_CALL_EXPRESSION_7 = env.storage();
        let mut METHOD_CALL_EXPRESSION_6 = METHOD_CALL_EXPRESSION_7.instance();
        let mut METHOD_CALL_EXPRESSION_1 = METHOD_CALL_EXPRESSION_6.get(&DataKey::Owner);
        let mut public_key: BytesN<32> = METHOD_CALL_EXPRESSION_1.unwrap();
        let mut METHOD_CALL_EXPRESSION_28 = env.crypto();
        let mut METHOD_CALL_ARG_2_18 = &signature_payload.into();
        METHOD_CALL_EXPRESSION_28.ed25519_verify(&public_key, METHOD_CALL_ARG_2_18, &signature);
    }
}


mod test;
