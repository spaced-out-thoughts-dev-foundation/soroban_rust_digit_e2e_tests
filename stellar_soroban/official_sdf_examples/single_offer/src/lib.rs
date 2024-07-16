#![no_std]
use soroban_sdk::{contract, contracttype, Address, token, contractimpl, Env, auth::Context, IntoVal, unwrap::UnwrapOptimized};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Offer,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offer {
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price: i128,
    pub buy_price: i128,
}

#[contract]
pub struct SingleOffer;

#[contractimpl]
impl SingleOffer {
    pub fn create(e: Env, seller: Address, sell_token: Address, buy_token: Address, sell_price: i128, buy_price: i128)  {
        let mut METHOD_CALL_EXPRESSION_6 = e.storage();
        let mut METHOD_CALL_EXPRESSION_5 = METHOD_CALL_EXPRESSION_6.instance();
        let mut CONDITIONAL_JUMP_ASSIGNMENT_0 = METHOD_CALL_EXPRESSION_5.has(&DataKey::Offer);
        if CONDITIONAL_JUMP_ASSIGNMENT_0 {
            panic!("offer is already created");
        }
        let BINARY_EXPRESSION_LEFT_17 = buy_price == 0;
        let BINARY_EXPRESSION_RIGHT_18 = sell_price == 0;
        let CONDITIONAL_JUMP_ASSIGNMENT_16 = BINARY_EXPRESSION_LEFT_17 || BINARY_EXPRESSION_RIGHT_18;
        if CONDITIONAL_JUMP_ASSIGNMENT_16 {
            panic!("zero price is not allowed");
        }
        seller.require_auth();
        let mut CALL_EXPRESSION_ARG_2_41 = &Offer{seller: seller, sell_token: sell_token, buy_token: buy_token, sell_price: sell_price, buy_price: buy_price};
        write_offer(&e, CALL_EXPRESSION_ARG_2_41);
    }


    pub fn trade(e: Env, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128)  {
        buyer.require_auth();
        let mut offer = load_offer(&e);
        let mut sell_token_client = token::Client::new(&e, &offer.sell_token);
        let mut buy_token_client = token::Client::new(&e, &offer.buy_token);
        let mut METHOD_CALL_EXPRESSION_94 = buy_token_amount.checked_mul(offer.sell_price);
        let mut BINARY_EXPRESSION_LEFT_92 = METHOD_CALL_EXPRESSION_94.unwrap_optimized();
        let mut sell_token_amount = BINARY_EXPRESSION_LEFT_92 / offer.buy_price;
        let CONDITIONAL_JUMP_ASSIGNMENT_109 = sell_token_amount < min_sell_token_amount;
        if CONDITIONAL_JUMP_ASSIGNMENT_109 {
            panic!("price is too low");
        }
        let mut contract = e.current_contract_address();
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);
    }


    pub fn withdraw(e: Env, token: Address, amount: i128)  {
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        let mut METHOD_CALL_EXPRESSION_200 = token::Client::new(&e, &token);
        let mut METHOD_CALL_ARG_1_183 = &e.current_contract_address();
        METHOD_CALL_EXPRESSION_200.transfer(METHOD_CALL_ARG_1_183, &offer.seller, &amount);
    }


    pub fn updt_price(e: Env, sell_price: i128, buy_price: i128)  {
        let BINARY_EXPRESSION_LEFT_214 = buy_price == 0;
        let BINARY_EXPRESSION_RIGHT_215 = sell_price == 0;
        let CONDITIONAL_JUMP_ASSIGNMENT_213 = BINARY_EXPRESSION_LEFT_214 || BINARY_EXPRESSION_RIGHT_215;
        if CONDITIONAL_JUMP_ASSIGNMENT_213 {
            panic!("zero price is not allowed");
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        write_offer(&e, &offer);
    }


    pub fn get_offer(e: Env) -> Offer {
        let mut Thing_to_return: Offer;
        Thing_to_return = load_offer(&e);
        return Thing_to_return;
    }
}

pub fn load_offer(e: &Env) -> Offer {
    let mut Thing_to_return: Offer;
    let mut METHOD_CALL_EXPRESSION_6 = e.storage();
    let mut METHOD_CALL_EXPRESSION_5 = METHOD_CALL_EXPRESSION_6.instance();
    let mut METHOD_CALL_EXPRESSION_0 = METHOD_CALL_EXPRESSION_5.get(&DataKey::Offer);
    Thing_to_return = METHOD_CALL_EXPRESSION_0.unwrap();
    return Thing_to_return;
}


pub fn write_offer(e: &Env, offer: &Offer)  {
    let mut METHOD_CALL_EXPRESSION_7 = e.storage();
    let mut METHOD_CALL_EXPRESSION_6 = METHOD_CALL_EXPRESSION_7.instance();
    METHOD_CALL_EXPRESSION_6.set(&DataKey::Offer, offer);
}



mod test;
