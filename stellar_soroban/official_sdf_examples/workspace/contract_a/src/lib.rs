#![no_std]
use soroban_sdk::{contract, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contract]
pub struct ContractA;

