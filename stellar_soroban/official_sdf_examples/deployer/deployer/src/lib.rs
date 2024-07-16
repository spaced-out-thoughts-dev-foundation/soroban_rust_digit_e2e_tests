#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, BytesN, String, Vec, Val, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(env: Env, deployer: Address, wasm_hash: BytesN<32>, salt: BytesN<32>, init_fn: String, init_args: Vec<Val>) -> (Address, Val) {
        let mut Thing_to_return: (Address, Val);
        let mut BINARY_EXPRESSION_RIGHT_3 = env.current_contract_address();
        let CONDITIONAL_JUMP_ASSIGNMENT_1 = deployer != BINARY_EXPRESSION_RIGHT_3;
        if CONDITIONAL_JUMP_ASSIGNMENT_1 {
            Thing_to_return = deployer.require_auth();
        }
        let mut METHOD_CALL_EXPRESSION_22 = env.deployer();
        let mut METHOD_CALL_EXPRESSION_17 = METHOD_CALL_EXPRESSION_22.with_address(deployer, salt);
        let mut deployed_address = METHOD_CALL_EXPRESSION_17.deploy(wasm_hash);
        let mut res: Val = env.invoke_contract(&deployed_address, &init_fn, init_args);
        Thing_to_return = (deployed_address, res);
        return Thing_to_return;
    }
}


mod test;
