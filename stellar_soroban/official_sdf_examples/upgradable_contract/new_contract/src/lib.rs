#![no_std]
use soroban_sdk::{contract, contracttype, contracterror, contractimpl, Env, Address, BytesN, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 1,
}

#[contract]
pub struct UpgradeableContract;

#[contractimpl]
impl UpgradeableContract {
    pub fn init(e: Env, admin: Address) -> Result<(), Error> {
        let mut Thing_to_return: Result<(), Error>;
        let mut METHOD_CALL_EXPRESSION_6 = e.storage();
        let mut METHOD_CALL_EXPRESSION_5 = METHOD_CALL_EXPRESSION_6.instance();
        let mut CONDITIONAL_JUMP_ASSIGNMENT_0 = METHOD_CALL_EXPRESSION_5.has(&DataKey::Admin);
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            let mut RETURN_VALUE_LABEL_14 = Err(Error::AlreadyInitialized);
            return RETURN_VALUE_LABEL_14;
        }
        let mut METHOD_CALL_EXPRESSION_31 = e.storage();
        let mut METHOD_CALL_EXPRESSION_30 = METHOD_CALL_EXPRESSION_31.instance();
        METHOD_CALL_EXPRESSION_30.set(&DataKey::Admin, &admin);
        let mut CALL_EXPRESSION_ARG_1_37 = ();
        Thing_to_return = Ok(CALL_EXPRESSION_ARG_1_37);
        return Thing_to_return;
    }


    pub fn version() -> i128 {
        let mut Thing_to_return: i128;
        return 2;
    }


    pub fn new_v2_fn() -> i128 {
        let mut Thing_to_return: i128;
        return 1010101;
    }


    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>)  {
        let mut METHOD_CALL_EXPRESSION_7 = e.storage();
        let mut METHOD_CALL_EXPRESSION_6 = METHOD_CALL_EXPRESSION_7.instance();
        let mut METHOD_CALL_EXPRESSION_1 = METHOD_CALL_EXPRESSION_6.get(&DataKey::Admin);
        let mut admin: Address = METHOD_CALL_EXPRESSION_1.unwrap();
        admin.require_auth();
        let mut METHOD_CALL_EXPRESSION_19 = e.deployer();
        METHOD_CALL_EXPRESSION_19.update_current_contract_wasm(new_wasm_hash);
    }
}
