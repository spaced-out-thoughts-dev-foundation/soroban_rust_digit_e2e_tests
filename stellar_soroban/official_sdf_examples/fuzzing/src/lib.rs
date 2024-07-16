#![no_std]
use soroban_sdk::{contract, contracttype, token, Address, Vec, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};



mod proptest {
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Init,
    Balance,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeBoundKind {
    Before,
    After,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(env: Env, from: Address, token: Address, amount: i128, claimants: Vec<Address>, time_bound: TimeBound)  {
        let mut CONDITIONAL_JUMP_ASSIGNMENT_0 = claimants.is_empty();
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("need more than 0 claimants");
        }
        let mut BINARY_EXPRESSION_LEFT_9 = claimants.len();
        let CONDITIONAL_JUMP_ASSIGNMENT_8 = BINARY_EXPRESSION_LEFT_9 > 10;
        if CONDITIONAL_JUMP_ASSIGNMENT_8 {
            panic!("too many claimants");
        }
        let mut CONDITIONAL_JUMP_ASSIGNMENT_20 = is_initialized(&env);
        if CONDITIONAL_JUMP_ASSIGNMENT_20 {
            panic!("contract has been already initialized");
        }
        from.require_auth();
        let mut METHOD_CALL_EXPRESSION_49 = token::Client::new(&env, &token);
        let mut METHOD_CALL_ARG_2_39 = &env.current_contract_address();
        METHOD_CALL_EXPRESSION_49.transfer(&from, METHOD_CALL_ARG_2_39, &amount);
        let mut METHOD_CALL_EXPRESSION_76 = env.storage();
        let mut METHOD_CALL_EXPRESSION_75 = METHOD_CALL_EXPRESSION_76.persistent();
        let mut METHOD_CALL_ARG_2_66 = &ClaimableBalance{token: token, amount: amount, claimants: time_bound, time_bound: claimants};
        METHOD_CALL_EXPRESSION_75.set(&DataKey::Balance, METHOD_CALL_ARG_2_66);
        let mut METHOD_CALL_EXPRESSION_92 = env.storage();
        let mut METHOD_CALL_EXPRESSION_91 = METHOD_CALL_EXPRESSION_92.persistent();
        let mut METHOD_CALL_ARG_2_86 = &();
        METHOD_CALL_EXPRESSION_91.set(&DataKey::Init, METHOD_CALL_ARG_2_86);
    }


    pub fn claim(env: Env, claimant: Address, amount: i128)  {
        claimant.require_auth();
        let mut METHOD_CALL_EXPRESSION_107 = env.storage();
        let mut METHOD_CALL_EXPRESSION_106 = METHOD_CALL_EXPRESSION_107.persistent();
        let mut METHOD_CALL_EXPRESSION_101 = METHOD_CALL_EXPRESSION_106.get(&DataKey::Balance);
        let mut claimable_balance: ClaimableBalance = METHOD_CALL_EXPRESSION_101.unwrap();
        let mut UNARY_ARGUMENT_115 = check_time_bound(&env, &claimable_balance.time_bound);
        let CONDITIONAL_JUMP_ASSIGNMENT_114 = !(UNARY_ARGUMENT_115);
        if CONDITIONAL_JUMP_ASSIGNMENT_114 {
            panic!("time predicate is not fulfilled");
        }
        let mut UNARY_ARGUMENT_142 = claimants.contains(&claimant);
        let CONDITIONAL_JUMP_ASSIGNMENT_141 = !(UNARY_ARGUMENT_142);
        if CONDITIONAL_JUMP_ASSIGNMENT_141 {
            panic!("claimant is not allowed to claim this balance");
        }
        let CONDITIONAL_JUMP_ASSIGNMENT_155 = amount > claimable_balance.amount;
        if CONDITIONAL_JUMP_ASSIGNMENT_155 {
            panic!("claimed amount greater than balance");
        }
        let mut METHOD_CALL_EXPRESSION_182 = token::Client::new(&env, &claimable_balance.token);
        let mut METHOD_CALL_ARG_1_168 = &env.current_contract_address();
        METHOD_CALL_EXPRESSION_182.transfer(METHOD_CALL_ARG_1_168, &claimant, &amount);
        let mut new_balance = claimable_balance.amount - amount;
        let CONDITIONAL_JUMP_ASSIGNMENT_206 = new_balance > 0;
        if CONDITIONAL_JUMP_ASSIGNMENT_206 {
            claimable_balance.amount = new_balance;
            let mut METHOD_CALL_EXPRESSION_231 = env.storage();
            let mut METHOD_CALL_EXPRESSION_230 = METHOD_CALL_EXPRESSION_231.persistent();
            METHOD_CALL_EXPRESSION_230.set(&DataKey::Balance, &claimable_balance);
        }
        else {
            let mut METHOD_CALL_EXPRESSION_245 = env.storage();
            let mut METHOD_CALL_EXPRESSION_244 = METHOD_CALL_EXPRESSION_245.persistent();
            METHOD_CALL_EXPRESSION_244.remove(&DataKey::Balance);
        }
    }
}

pub fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let mut Thing_to_return: bool;
    let mut METHOD_CALL_EXPRESSION_0 = env.ledger();
    let mut ledger_timestamp = METHOD_CALL_EXPRESSION_0.timestamp();
    let mut THING_TO_COMPARE_AGAINST_5 = time_bound;
    let CONDITIONAL_JUMP_CHECK_10 = time_bound.kind == TimeBoundKind::Before;
    let CONDITIONAL_JUMP_CHECK_23 = THING_TO_COMPARE_AGAINST_5 == TimeBoundKind::After;
    if CONDITIONAL_JUMP_CHECK_10 {
        let Thing_to_return = ledger_timestamp <= time_bound.timestamp;
    }
    else if CONDITIONAL_JUMP_CHECK_23 {
        let Thing_to_return = ledger_timestamp >= time_bound.timestamp;
    }
    return Thing_to_return;
}


pub fn is_initialized(env: &Env) -> bool {
    let mut Thing_to_return: bool;
    let mut METHOD_CALL_EXPRESSION_5 = env.storage();
    let mut METHOD_CALL_EXPRESSION_4 = METHOD_CALL_EXPRESSION_5.persistent();
    Thing_to_return = METHOD_CALL_EXPRESSION_4.has(&DataKey::Init);
    return Thing_to_return;
}



mod test;
