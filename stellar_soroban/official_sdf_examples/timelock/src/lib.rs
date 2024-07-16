#![no_std]
use soroban_sdk::{contract, contracttype, token, Address, Vec, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

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
        let mut BINARY_EXPRESSION_LEFT_1 = claimants.len();
        let CONDITIONAL_JUMP_ASSIGNMENT_0 = BINARY_EXPRESSION_LEFT_1 > 10;
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("too many claimants");
        }
        let mut CONDITIONAL_JUMP_ASSIGNMENT_12 = is_initialized(&env);
        if CONDITIONAL_JUMP_ASSIGNMENT_12 {
            panic!("contract has been already initialized");
        }
        from.require_auth();
        let mut METHOD_CALL_EXPRESSION_41 = token::Client::new(&env, &token);
        let mut METHOD_CALL_ARG_2_31 = &env.current_contract_address();
        METHOD_CALL_EXPRESSION_41.transfer(&from, METHOD_CALL_ARG_2_31, &amount);
        let mut METHOD_CALL_EXPRESSION_68 = env.storage();
        let mut METHOD_CALL_EXPRESSION_67 = METHOD_CALL_EXPRESSION_68.instance();
        let mut METHOD_CALL_ARG_2_58 = &ClaimableBalance{token: token, amount: amount, claimants: time_bound, time_bound: claimants};
        METHOD_CALL_EXPRESSION_67.set(&DataKey::Balance, METHOD_CALL_ARG_2_58);
        let mut METHOD_CALL_EXPRESSION_84 = env.storage();
        let mut METHOD_CALL_EXPRESSION_83 = METHOD_CALL_EXPRESSION_84.instance();
        let mut METHOD_CALL_ARG_2_78 = &();
        METHOD_CALL_EXPRESSION_83.set(&DataKey::Init, METHOD_CALL_ARG_2_78);
    }


    pub fn claim(env: Env, claimant: Address)  {
        claimant.require_auth();
        let mut METHOD_CALL_EXPRESSION_99 = env.storage();
        let mut METHOD_CALL_EXPRESSION_98 = METHOD_CALL_EXPRESSION_99.instance();
        let mut METHOD_CALL_EXPRESSION_93 = METHOD_CALL_EXPRESSION_98.get(&DataKey::Balance);
        let mut claimable_balance: ClaimableBalance = METHOD_CALL_EXPRESSION_93.unwrap();
        let mut UNARY_ARGUMENT_107 = check_time_bound(&env, &claimable_balance.time_bound);
        let CONDITIONAL_JUMP_ASSIGNMENT_106 = !(UNARY_ARGUMENT_107);
        if CONDITIONAL_JUMP_ASSIGNMENT_106 {
            panic!("time predicate is not fulfilled");
        }
        let mut UNARY_ARGUMENT_134 = claimants.contains(&claimant);
        let CONDITIONAL_JUMP_ASSIGNMENT_133 = !(UNARY_ARGUMENT_134);
        if CONDITIONAL_JUMP_ASSIGNMENT_133 {
            panic!("claimant is not allowed to claim this balance");
        }
        let mut METHOD_CALL_EXPRESSION_164 = token::Client::new(&env, &claimable_balance.token);
        let mut METHOD_CALL_ARG_1_147 = &env.current_contract_address();
        METHOD_CALL_EXPRESSION_164.transfer(METHOD_CALL_ARG_1_147, &claimant, &claimable_balance.amount);
        let mut METHOD_CALL_EXPRESSION_185 = env.storage();
        let mut METHOD_CALL_EXPRESSION_184 = METHOD_CALL_EXPRESSION_185.instance();
        METHOD_CALL_EXPRESSION_184.remove(&DataKey::Balance);
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
    let mut METHOD_CALL_EXPRESSION_4 = METHOD_CALL_EXPRESSION_5.instance();
    Thing_to_return = METHOD_CALL_EXPRESSION_4.has(&DataKey::Init);
    return Thing_to_return;
}



mod test;
