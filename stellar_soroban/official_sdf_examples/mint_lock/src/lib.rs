#![no_std]
use soroban_sdk::{contract, contracterror, contracttype, Address, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotAuthorizedMinter = 1,
    DailyLimitInsufficient = 2,
    NegativeAmount = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    Admin,
    Minter(Address,  Address),
    MinterStats(Address,  Address,  i128,  i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinterConfig {
    pub limit: i128,
    pub epoch_length: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinterStats {
    pub consumed_limit: i128,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set_admin(env: Env, new_admin: Address)  {
        let mut METHOD_CALL_EXPRESSION_7 = env.storage();
        let mut METHOD_CALL_EXPRESSION_6 = METHOD_CALL_EXPRESSION_7.instance();
        let mut INPUT_VALUE_NAME_FOR_LET_1 = METHOD_CALL_EXPRESSION_6.get(&StorageKey::Admin);
        if let Some(admin) = INPUT_VALUE_NAME_FOR_LET_1 {
            admin.require_auth();
        }
        let mut METHOD_CALL_EXPRESSION_29 = env.storage();
        let mut METHOD_CALL_EXPRESSION_28 = METHOD_CALL_EXPRESSION_29.instance();
        METHOD_CALL_EXPRESSION_28.set(&StorageKey::Admin, &new_admin);
    }


    pub fn admin(env: Env) -> Address {
        let mut Thing_to_return: Address;
        let mut METHOD_CALL_EXPRESSION_41 = env.storage();
        let mut METHOD_CALL_EXPRESSION_40 = METHOD_CALL_EXPRESSION_41.instance();
        let mut METHOD_CALL_EXPRESSION_35 = METHOD_CALL_EXPRESSION_40.get(&StorageKey::Admin);
        Thing_to_return = METHOD_CALL_EXPRESSION_35.unwrap();
        return Thing_to_return;
    }


    pub fn set_minter(env: Env, contract: Address, minter: Address, config: MinterConfig)  {
        let mut CALL_EXPRESSION_ARG_1_49 = env.clone();
        let mut METHOD_CALL_EXPRESSION_48 = Self::admin(CALL_EXPRESSION_ARG_1_49);
        METHOD_CALL_EXPRESSION_48.require_auth();
        let mut METHOD_CALL_EXPRESSION_72 = env.storage();
        let mut METHOD_CALL_EXPRESSION_71 = METHOD_CALL_EXPRESSION_72.persistent();
        let mut METHOD_CALL_ARG_1_57 = &StorageKey::Minter(contract, minter);
        METHOD_CALL_EXPRESSION_71.set(METHOD_CALL_ARG_1_57, &config);
    }


    pub fn minter(env: Env, contract: Address, minter: Address) -> Result<(MinterConfig, i128, MinterStats), Error> {
        let mut Thing_to_return: Result<(MinterConfig, i128, MinterStats), Error>;
        let mut METHOD_CALL_EXPRESSION_96 = env.storage();
        let mut METHOD_CALL_EXPRESSION_95 = METHOD_CALL_EXPRESSION_96.persistent();
        let mut CALL_EXPRESSION_ARG_1_84 = contract.clone();
        let mut CALL_EXPRESSION_ARG_2_88 = minter.clone();
        let mut METHOD_CALL_ARG_1_81 = &StorageKey::Minter(CALL_EXPRESSION_ARG_1_84, CALL_EXPRESSION_ARG_2_88);
        let mut METHOD_CALL_EXPRESSION_80 = METHOD_CALL_EXPRESSION_95.get(METHOD_CALL_ARG_1_81);
        let mut config = METHOD_CALL_EXPRESSION_80.ok_or(Error::NotAuthorizedMinter);
        let mut METHOD_CALL_EXPRESSION_105 = env.ledger();
        let mut BINARY_EXPRESSION_LEFT_103 = METHOD_CALL_EXPRESSION_105.sequence();
        let mut epoch = BINARY_EXPRESSION_LEFT_103 / config.epoch_length;
        let mut METHOD_CALL_EXPRESSION_138 = env.storage();
        let mut METHOD_CALL_EXPRESSION_137 = METHOD_CALL_EXPRESSION_138.temporary();
        let mut CALL_EXPRESSION_ARG_1_119 = contract.clone();
        let mut CALL_EXPRESSION_ARG_2_123 = minter.clone();
        let mut METHOD_CALL_ARG_1_116 = &StorageKey::MinterStats(CALL_EXPRESSION_ARG_1_119, CALL_EXPRESSION_ARG_2_123, config.epoch_length, epoch);
        let mut METHOD_CALL_EXPRESSION_115 = METHOD_CALL_EXPRESSION_137.get(METHOD_CALL_ARG_1_116);
        let mut stats = METHOD_CALL_EXPRESSION_115.unwrap_or_default();
        let mut CALL_EXPRESSION_ARG_1_145 = (config, epoch, stats);
        Thing_to_return = Ok(CALL_EXPRESSION_ARG_1_145);
        return Thing_to_return;
    }


    pub fn mint(env: Env, contract: Address, minter: Address, to: Address, amount: i128) -> Result<(), Error> {
        let mut Thing_to_return: Result<(), Error>;
        let mut METHOD_CALL_EXPRESSION_159 = (&, contract, &, to, amount);
        let mut METHOD_CALL_ARG_1_154 = METHOD_CALL_EXPRESSION_159.into_val(&env);
        minter.require_auth_for_args(METHOD_CALL_ARG_1_154);
        let CONDITIONAL_JUMP_ASSIGNMENT_173 = amount < 0;
        if CONDITIONAL_JUMP_ASSIGNMENT_173 {
            let mut RETURN_VALUE_LABEL_181 = Err(Error::NegativeAmount);
            return RETURN_VALUE_LABEL_181;
        }
        let mut CALL_EXPRESSION_ARG_1_189 = env.clone();
        let mut admin = Self::admin(CALL_EXPRESSION_ARG_1_189);
        let CONDITIONAL_JUMP_ASSIGNMENT_196 = admin != minter;
        if CONDITIONAL_JUMP_ASSIGNMENT_196 {
            let mut METHOD_CALL_EXPRESSION_221 = env.storage();
            let mut METHOD_CALL_EXPRESSION_220 = METHOD_CALL_EXPRESSION_221.persistent();
            let mut CALL_EXPRESSION_ARG_1_209 = contract.clone();
            let mut CALL_EXPRESSION_ARG_2_213 = minter.clone();
            let mut METHOD_CALL_ARG_1_206 = &StorageKey::Minter(CALL_EXPRESSION_ARG_1_209, CALL_EXPRESSION_ARG_2_213);
            let mut TRY_ASSIGN_RESULT_204 = METHOD_CALL_EXPRESSION_220.get(METHOD_CALL_ARG_1_206);
            if let TRY_ASSIGN_RESULT_204 = Some(config) {
            }
            let mut METHOD_CALL_EXPRESSION_243 = env.ledger();
            let mut BINARY_EXPRESSION_LEFT_241 = METHOD_CALL_EXPRESSION_243.sequence();
            let mut epoch = BINARY_EXPRESSION_LEFT_241 / config.epoch_length;
            let mut CALL_EXPRESSION_ARG_1_253 = contract.clone();
            let mut CALL_EXPRESSION_ARG_2_257 = minter.clone();
            let mut minter_stats_key = StorageKey::MinterStats(CALL_EXPRESSION_ARG_1_253, CALL_EXPRESSION_ARG_2_257, config.epoch_length, epoch);
            let mut METHOD_CALL_EXPRESSION_277 = env.storage();
            let mut METHOD_CALL_EXPRESSION_276 = METHOD_CALL_EXPRESSION_277.temporary();
            let mut METHOD_CALL_EXPRESSION_271 = METHOD_CALL_EXPRESSION_276.get(&minter_stats_key);
            let mut minter_stats = METHOD_CALL_EXPRESSION_271.unwrap_or_default();
            let mut consumed_limit = minter_stats.consumed_limit + amount;
            let mut new_minter_stats = MinterStats{consumed_limit: consumed_limit};
            let CONDITIONAL_JUMP_ASSIGNMENT_294 = new_minter_stats.consumed_limit > config.limit;
            if CONDITIONAL_JUMP_ASSIGNMENT_294 {
                let mut RETURN_VALUE_LABEL_308 = Err(Error::DailyLimitInsufficient);
                return RETURN_VALUE_LABEL_308;
            }
            let mut METHOD_CALL_EXPRESSION_325 = env.storage();
            let mut METHOD_CALL_EXPRESSION_324 = METHOD_CALL_EXPRESSION_325.temporary();
            METHOD_CALL_EXPRESSION_324.set(&minter_stats_key, &new_minter_stats);
            let mut METHOD_CALL_EXPRESSION_347 = env.storage();
            let mut METHOD_CALL_EXPRESSION_346 = METHOD_CALL_EXPRESSION_347.temporary();
            let mut METHOD_CALL_ARG_3_337 = epoch & config.epoch_length;
            Thing_to_return = METHOD_CALL_EXPRESSION_346.extend_ttl(&minter_stats_key, 0, METHOD_CALL_ARG_3_337);
        }
            else {
                let mut RETURN_VALUE_LABEL_233 = Err(Error::NotAuthorizedMinter);
                return RETURN_VALUE_LABEL_233;
            }
        let mut client = MintClient::new(&env, &contract);
        client.mint(&to, &amount);
        let mut CALL_EXPRESSION_ARG_1_376 = ();
        Thing_to_return = Ok(CALL_EXPRESSION_ARG_1_376);
        return Thing_to_return;
    }
}


mod test;
