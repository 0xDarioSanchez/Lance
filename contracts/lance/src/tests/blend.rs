#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events},
    Env, Address, Vec,
};

use crate::methods::blend::*;

use blend_contract_sdk::pool;
use crate::methods::balance::*;
use crate::storage::{
    error::Error, storage::DataKey,
};


const TOKEN: DataKey = DataKey::Token;
const BLEND_POOL: DataKey = DataKey::BlendPool;
const ADMIN: DataKey = DataKey::Admin; 
const TOTAL_PRINCIPAL: DataKey = DataKey::TotalPrincipal;

// Blend pool request type codes
const BLEND_SUPPLY_REQUEST: u32 = 0;
const BLEND_WITHDRAW_REQUEST: u32 = 1;
const BLEND_BORROW_REQUEST: u32 = 2;
const BLEND_REPAY_REQUEST: u32 = 3;
const DEFAULT_RESERVE_ID: u32 = 0; 

#[test]
fn test_set_and_get_blend_pool() {
    let env = Env::default();

    let pool_addr = Address::generate(&env);
    set_blend_pool(&env, &pool_addr);

    let stored = get_blend_pool(&env).unwrap();
    assert_eq!(stored, pool_addr);
}

#[test]
fn test_lend_to_blend_no_tokens() {
    let env = Env::default();

    // setup admin + storage
    let admin = Address::generate(&env);
    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &Address::generate(&env));
    env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

    // balance is 0 → should fail
    let result = lend_to_blend(env.clone());
    assert!(matches!(result, Err(Error::NoTokensToLend)));
}

#[test]
fn test_withdraw_from_blend_no_position() {
    let env = Env::default();

    let admin = Address::generate(&env);
    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &Address::generate(&env));
    env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

    // no supply position → should fail
    let result = withdraw_from_blend(env.clone());
    assert!(matches!(result, Err(Error::NoPositionInBlend)));
}

#[test]
fn test_withdraw_amount_invalid() {
    let env = Env::default();

    let admin = Address::generate(&env);
    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &Address::generate(&env));
    env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

    // amount <= 0 should fail
    let result = withdraw_amount_from_blend(env.clone(), 0);
    assert!(matches!(result, Err(Error::InvalidAmount)));
}
