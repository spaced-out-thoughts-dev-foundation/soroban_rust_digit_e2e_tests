#![no_std]
use soroban_sdk::{contract, contracttype, Address, contractimpl, Env, token, Vec, auth::Context, IntoVal, unwrap::UnwrapOptimized};



mod atomic_swap {
	soroban_sdk::contractimport!(
		file = "../atomic_swap/target/wasm32-unknown-unknown/release/soroban_atomic_swap_contract.wasm"
	);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapSpec {
    pub address: Address,
    pub amount: i128,
    pub min_recv: i128,
}

#[contract]
pub struct AtomicMultiSwapContract;

#[contractimpl]
impl AtomicMultiSwapContract {
    pub fn multi_swap(env: Env, swap_contract: Address, token_a: Address, token_b: Address, swaps_a: Vec<SwapSpec>, swaps_b: Vec<SwapSpec>)  {
        let mut swap_client = atomic_swap::Client::new(&env, &swap_contract);
        let mut ITERATOR_12 = swaps_a.iter();
        let mut OPTION_acc_a = ITERATOR_12.next();
        while let Some(acc_a) = OPTION_acc_a {
            let mut RANGE_END_24 = swaps_b.len();
            let mut ITERATOR_21 = 0..RANGE_END_24;
            let mut OPTION_i = ITERATOR_21.next();
            while let Some(i) = OPTION_i {
                let mut METHOD_CALL_EXPRESSION_34 = swaps_b.get(i);
                let mut acc_b = METHOD_CALL_EXPRESSION_34.unwrap();
                let BINARY_EXPRESSION_LEFT_42 = acc_a.amount >= acc_b.min_recv;
                let BINARY_EXPRESSION_RIGHT_43 = acc_a.min_recv <= acc_b.amount;
                let CONDITIONAL_JUMP_ASSIGNMENT_41 = BINARY_EXPRESSION_LEFT_42 && BINARY_EXPRESSION_RIGHT_43;
                if CONDITIONAL_JUMP_ASSIGNMENT_41 {
                    let mut METHOD_CALL_EXPRESSION_70 = swap_client.try_swap(&acc_a.address, &acc_b.address, &token_a, &token_b, &acc_a.amount, &acc_a.min_recv, &acc_b.amount, &acc_b.min_recv);
                    let mut CONDITIONAL_JUMP_ASSIGNMENT_69 = METHOD_CALL_EXPRESSION_70.is_ok();
                    if CONDITIONAL_JUMP_ASSIGNMENT_69 {
                        swaps_b.remove(i);
                        break;
                    }
                }
                OPTION_i = ITERATOR_21.next();
                                                                                    }
                OPTION_acc_a = ITERATOR_12.next();
        }
    }
}


mod test;
