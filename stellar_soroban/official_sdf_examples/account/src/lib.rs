#![no_std]
use soroban_sdk::{contract, contracttype, BytesN, Address, Symbol, symbol_short, contractimpl, Env, Vec, token, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccSignature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    SignerCnt,
    Signer(BytesN<32>),
    SpendLimit(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccError {
    NotEnoughSigners = 1,
    NegativeAmount = 2,
    BadSignatureOrder = 3,
    UnknownSigner = 4,
    InvalidContext = 5,
}

const TRANSFER_FN: Symbol = symbol_short!("transfer");
#[contract]
pub struct AccountContract;

#[contractimpl]
impl AccountContract {
    pub fn init(env: Env, signers: Vec<BytesN<32>>)  {
        let mut ITERATOR_1 = signers.iter();
        let mut OPTION_signer = ITERATOR_1.next();
        while let Some(signer) = OPTION_signer {
            let mut METHOD_CALL_EXPRESSION_24 = env.storage();
            let mut METHOD_CALL_EXPRESSION_23 = METHOD_CALL_EXPRESSION_24.instance();
            let mut METHOD_CALL_ARG_1_10 = &DataKey::Signer(signer);
            let mut METHOD_CALL_ARG_2_18 = &();
            METHOD_CALL_EXPRESSION_23.set(METHOD_CALL_ARG_1_10, METHOD_CALL_ARG_2_18);
            OPTION_signer = ITERATOR_1.next();
        }
        let mut METHOD_CALL_EXPRESSION_43 = env.storage();
        let mut METHOD_CALL_EXPRESSION_42 = METHOD_CALL_EXPRESSION_43.instance();
        let mut METHOD_CALL_ARG_2_36 = &signers.len();
        METHOD_CALL_EXPRESSION_42.set(&DataKey::SignerCnt, METHOD_CALL_ARG_2_36);
    }


    pub fn add_limit(env: Env, token: Address, limit: i128)  {
        let mut METHOD_CALL_EXPRESSION_49 = env.current_contract_address();
        METHOD_CALL_EXPRESSION_49.require_auth();
        let mut METHOD_CALL_EXPRESSION_67 = env.storage();
        let mut METHOD_CALL_EXPRESSION_66 = METHOD_CALL_EXPRESSION_67.instance();
        let mut METHOD_CALL_ARG_1_54 = &DataKey::SpendLimit(token);
        METHOD_CALL_EXPRESSION_66.set(METHOD_CALL_ARG_1_54, &limit);
    }
}

pub fn authenticate(env: &Env, signature_payload: &BytesN<32>, signatures: &Vec<AccSignature>) -> Result<(), AccError> {
    let mut Thing_to_return: Result<(), AccError>;
    let mut RANGE_END_4 = signatures.len();
    let mut ITERATOR_1 = 0..RANGE_END_4;
    let mut OPTION_i = ITERATOR_1.next();
    while let Some(i) = OPTION_i {
        let mut signature = signatures.get_unchecked(i);
        let CONDITIONAL_JUMP_ASSIGNMENT_19 = i > 0;
        if CONDITIONAL_JUMP_ASSIGNMENT_19 {
            let mut METHOD_CALL_ARG_1_27 = i - 1;
            let mut prev_signature = signatures.get_unchecked(METHOD_CALL_ARG_1_27);
            let CONDITIONAL_JUMP_ASSIGNMENT_36 = prev_signature.public_key >= signature.public_key;
            if CONDITIONAL_JUMP_ASSIGNMENT_36 {
                let mut RETURN_VALUE_LABEL_50 = Err(AccError::BadSignatureOrder);
                return RETURN_VALUE_LABEL_50;
            }
        }
        let mut METHOD_CALL_EXPRESSION_75 = env.storage();
        let mut METHOD_CALL_EXPRESSION_74 = METHOD_CALL_EXPRESSION_75.instance();
        let mut CALL_EXPRESSION_ARG_1_64 = signature.public_key.clone();
        let mut METHOD_CALL_ARG_1_61 = &DataKey::Signer(CALL_EXPRESSION_ARG_1_64);
        let mut UNARY_ARGUMENT_60 = METHOD_CALL_EXPRESSION_74.has(METHOD_CALL_ARG_1_61);
        let CONDITIONAL_JUMP_ASSIGNMENT_59 = !(UNARY_ARGUMENT_60);
        if CONDITIONAL_JUMP_ASSIGNMENT_59 {
            let mut RETURN_VALUE_LABEL_84 = Err(AccError::UnknownSigner);
            return RETURN_VALUE_LABEL_84;
        }
        let mut METHOD_CALL_EXPRESSION_114 = env.crypto();
        let mut METHOD_CALL_EXPRESSION_102 = signature_payload.clone();
        let mut METHOD_CALL_ARG_2_99 = &METHOD_CALL_EXPRESSION_102.into();
        Thing_to_return = METHOD_CALL_EXPRESSION_114.ed25519_verify(&signature.public_key, METHOD_CALL_ARG_2_99, &signature.signature);
        OPTION_i = ITERATOR_1.next();
    }
    let mut CALL_EXPRESSION_ARG_1_121 = ();
    Thing_to_return = Ok(CALL_EXPRESSION_ARG_1_121);
    return Thing_to_return;
}


pub fn verify_authorization_policy(env: &Env, context: &Context, curr_contract: &Address, all_signed: bool, spend_left_per_token: &HashMap<Address, i128>) -> Result<(), AccError> {
    let mut Thing_to_return: Result<(), AccError>;
    let mut THING_TO_COMPARE_AGAINST_0 = context;
    let CONDITIONAL_JUMP_CHECK_2 = context == Context::Contract(c;
    let CONDITIONAL_JUMP_CHECK_36 = THING_TO_COMPARE_AGAINST_0 == Context::CreateContractHostFn(_;
    if CONDITIONAL_JUMP_CHECK_2 {
        let CONDITIONAL_JUMP_ASSIGNMENT_6 = &c.contract == curr_contract;
        if CONDITIONAL_JUMP_ASSIGNMENT_6 {
            let CONDITIONAL_JUMP_ASSIGNMENT_19 = !(all_signed);
            if CONDITIONAL_JUMP_ASSIGNMENT_19 {
                let mut RETURN_VALUE_LABEL_25 = Err(AccError::NotEnoughSigners);
                return RETURN_VALUE_LABEL_25;
            }
        }
    }
    else if CONDITIONAL_JUMP_CHECK_36 {
        let mut RETURN_VALUE_LABEL_40 = Err(AccError::InvalidContext);
        return RETURN_VALUE_LABEL_40;
    }
    let BINARY_EXPRESSION_LEFT_49 = contract_context.fn_name != TRANSFER_FN;
    let mut BINARY_EXPRESSION_RIGHT_60 = Symbol::new(env, "approve");
    let BINARY_EXPRESSION_RIGHT_50 = contract_context.fn_name != BINARY_EXPRESSION_RIGHT_60;
    let CONDITIONAL_JUMP_ASSIGNMENT_48 = BINARY_EXPRESSION_LEFT_49 && BINARY_EXPRESSION_RIGHT_50;
    if CONDITIONAL_JUMP_ASSIGNMENT_48 {
        let mut CALL_EXPRESSION_ARG_1_77 = ();
        let mut RETURN_VALUE_LABEL_76 = Ok(CALL_EXPRESSION_ARG_1_77);
        return RETURN_VALUE_LABEL_76;
    }
    let mut METHOD_CALL_ARG_1_87 = contract_context.contract.clone();
    let mut INPUT_VALUE_NAME_FOR_LET_86 = spend_left_per_token.get(METHOD_CALL_ARG_1_87);
    if let Some(spend_left = INPUT_VALUE_NAME_FOR_LET_86 {
        let mut spend_left: Option<i128> = Some(spend_left);
    }
    else {
        let mut METHOD_CALL_EXPRESSION_124 = env.storage();
        let mut METHOD_CALL_EXPRESSION_123 = METHOD_CALL_EXPRESSION_124.instance();
        let mut CALL_EXPRESSION_ARG_1_113 = contract_context.contract.clone();
        let mut METHOD_CALL_ARG_1_110 = &DataKey::SpendLimit(CALL_EXPRESSION_ARG_1_113);
        let mut INPUT_VALUE_NAME_FOR_LET_109 = METHOD_CALL_EXPRESSION_123.get(METHOD_CALL_ARG_1_110);
        if let Some(limit_left = INPUT_VALUE_NAME_FOR_LET_109 {
            let mut spend_left: Option<i128> = Some(limit_left);
        }
        else {
            let mut spend_left: Option<i128> = None;
        }
    }
    if let Some(spend_left = spend_left {
        let mut METHOD_CALL_EXPRESSION_154 = contract_context.args.get(2);
        let mut METHOD_CALL_EXPRESSION_153 = METHOD_CALL_EXPRESSION_154.unwrap();
        let mut METHOD_CALL_EXPRESSION_150 = METHOD_CALL_EXPRESSION_153.try_into_val(env);
        let mut spent: i128 = METHOD_CALL_EXPRESSION_150.unwrap();
        let CONDITIONAL_JUMP_ASSIGNMENT_166 = spent < 0;
        if CONDITIONAL_JUMP_ASSIGNMENT_166 {
            let mut RETURN_VALUE_LABEL_174 = Err(AccError::NegativeAmount);
            return RETURN_VALUE_LABEL_174;
        }
        let BINARY_EXPRESSION_LEFT_183 = !(all_signed);
        let BINARY_EXPRESSION_RIGHT_184 = spent > spend_left;
        let CONDITIONAL_JUMP_ASSIGNMENT_182 = BINARY_EXPRESSION_LEFT_183 && BINARY_EXPRESSION_RIGHT_184;
        if CONDITIONAL_JUMP_ASSIGNMENT_182 {
            let mut RETURN_VALUE_LABEL_196 = Err(AccError::NotEnoughSigners);
            return RETURN_VALUE_LABEL_196;
        }
        let mut METHOD_CALL_ARG_1_204 = contract_context.contract.clone();
        let mut METHOD_CALL_ARG_2_211 = spend_left - spent;
        Thing_to_return = spend_left_per_token.set(METHOD_CALL_ARG_1_204, METHOD_CALL_ARG_2_211);
    }
    let mut CALL_EXPRESSION_ARG_1_221 = ();
    Thing_to_return = Ok(CALL_EXPRESSION_ARG_1_221);
    return Thing_to_return;
}



mod test;
