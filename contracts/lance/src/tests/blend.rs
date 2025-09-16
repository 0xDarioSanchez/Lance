#![cfg(test)]

use soroban_sdk::{
    contract, contractimpl, testutils::Address as TestAddress, Address, Env, Map, Vec,
};
use crate::methods::blend::*;
use crate::storage::{error::Error, storage::DataKey};
use blend_contract_sdk::pool;

// ------------------- Constants -------------------
const TOKEN: DataKey = DataKey::Token;
const BLEND_POOL: DataKey = DataKey::BlendPool;
const ADMIN: DataKey = DataKey::Admin;
const TOTAL_PRINCIPAL: DataKey = DataKey::TotalPrincipal;

const BLEND_SUPPLY_REQUEST: u32 = 0;
const BLEND_WITHDRAW_REQUEST: u32 = 1;

// ---------------- MOCK TOKEN ----------------
//
// This mock implements a minimal token interface that TokenClient typically expects:
// - balance(Address) -> i128
// - mint(Address, i128)
// - burn(Address, i128)
// - transfer(Address, Address, i128)
// - transfer_from(Address, Address, Address, i128)
// - allowance(Address, Address) -> i128
// - approve(Address, Address, i128)
//
// Storage layout used here:
// - balances keyed by Address -> i128
// - allowances keyed by (owner, spender) -> i128
//
#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    // balance(addr) -> i128
    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&addr).unwrap_or(0)
    }

    // mint(to, amount)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let mut bal: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        bal = bal.saturating_add(amount);
        env.storage().persistent().set(&to, &bal);
    }

    // burn(from, amount)
    pub fn burn(env: Env, from: Address, amount: i128) {
        let mut bal: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        bal = bal.saturating_sub(amount);
        env.storage().persistent().set(&from, &bal);
    }

    // transfer(from, to, amount)
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // NOTE: skipping auth checks for tests
        let mut from_bal: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        from_bal = from_bal.saturating_sub(amount);
        env.storage().persistent().set(&from, &from_bal);

        let mut to_bal: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        to_bal = to_bal.saturating_add(amount);
        env.storage().persistent().set(&to, &to_bal);
    }

    // transfer_from(spender, from, to, amount)
    pub fn transfer_from(env: Env, _spender: Address, from: Address, to: Address, amount: i128) {
        // reduce allowance, move tokens
        let mut allowance: i128 = env
            .storage()
            .persistent()
            .get(&(from.clone(), _spender.clone()))
            .unwrap_or(0);
        allowance = allowance.saturating_sub(amount);
        env.storage().persistent().set(&(from.clone(), _spender.clone()), &allowance);

        let mut from_bal: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        from_bal = from_bal.saturating_sub(amount);
        env.storage().persistent().set(&from, &from_bal);

        let mut to_bal: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        to_bal = to_bal.saturating_add(amount);
        env.storage().persistent().set(&to, &to_bal);
    }

    // allowance(owner, spender) -> i128
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&(owner, spender))
            .unwrap_or(0)
    }

    // approve(owner, spender, amount)
    // In the real token contract owner would call approve and require_auth(owner).
    // For tests we accept (owner, spender, amount) signature and set allowance directly.
    pub fn approve(env: Env, _owner: Address, spender: Address, amount: i128) {
        // NOTE: ignoring auth: tests will set allowances directly or call via client which supplies owner
        env.storage()
            .persistent()
            .set(&(_owner, spender), &amount);
    }
}

// ---------------- MOCK BLEND POOL ----------------
// Implements pool functions expected by pool::Client:
// - get_positions(Address) -> pool::Positions
// - submit_with_allowance(from, spender, to, Vec<pool::Request>)
//
#[contract]
pub struct MockPool;

#[contractimpl]
impl MockPool {
    pub fn get_positions(env: Env, addr: Address) -> pool::Positions {
        let mut supply = Map::new(&env);

        // We store a single supply entry at persistent key: (addr, 0u32)
        let amount: i128 = env
            .storage()
            .persistent()
            .get(&(addr.clone(), 0u32))
            .unwrap_or(0);
        if amount > 0 {
            supply.set(0u32, amount);
        }

        pool::Positions {
            supply,
            collateral: Map::new(&env),
            liabilities: Map::new(&env),
        }
    }

    // submit_with_allowance(from, spender, to, reqs)
    // This mock will apply only supply/withdraw request types used in your tests (0 and 1).
    pub fn submit_with_allowance(
        env: Env,
        from: Address,
        _spender: Address,
        _to: Address,
        reqs: Vec<pool::Request>,
    ) {
        for req in reqs.iter() {
            match req.request_type {
                BLEND_SUPPLY_REQUEST => {
                    let mut bal: i128 =
                        env.storage().persistent().get(&(from.clone(), 0u32)).unwrap_or(0);
                    bal = bal.saturating_add(req.amount);
                    env.storage().persistent().set(&(from.clone(), 0u32), &bal);
                }
                BLEND_WITHDRAW_REQUEST => {
                    let mut bal: i128 =
                        env.storage().persistent().get(&(from.clone(), 0u32)).unwrap_or(0);
                    bal = bal.saturating_sub(req.amount);
                    env.storage().persistent().set(&(from.clone(), 0u32), &bal);
                }
                _ => {
                    // ignore other request types for tests
                }
            }
        }
    }
}

// ------------------- TESTS -------------------
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
    let admin = Address::generate(&env);

    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &Address::generate(&env));
    env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

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

    let result = withdraw_amount_from_blend(env.clone(), 0);
    assert!(matches!(result, Err(Error::InvalidAmount)));
}

#[test]
fn test_lend_to_blend_success() {
    let env = Env::default();

    // Register mocks
    let token_id = env.register(MockToken, ());
    let pool_id = env.register(MockPool, ());

    // Setup storage
    let admin = Address::generate(&env);
    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &token_id);
    env.storage().instance().set(&BLEND_POOL, &pool_id);

    // Mint tokens to contract using generated client
    // The client call in tests uses MockTokenClient::new(&env, &token_id).mint(...)
    MockTokenClient::new(&env, &token_id).mint(&env.current_contract_address(), &1000);

    // Run function
    let result = lend_to_blend(env.clone()).unwrap();
    assert_eq!(result, 1000);
}

#[test]
fn test_withdraw_from_blend_success() {
    let env = Env::default();

    // Register mocks
    let token_id = env.register(MockToken, ());
    let pool_id = env.register(MockPool, ());

    let admin = Address::generate(&env);
    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &token_id);
    env.storage().instance().set(&BLEND_POOL, &pool_id);

    // Pretend user already supplied 500 into pool
    env.storage()
        .persistent()
        .set(&(env.current_contract_address(), 0u32), &500i128);

    let result = withdraw_from_blend(env.clone()).unwrap();
    assert_eq!(result, 500);
}

#[test]
fn test_withdraw_amount_too_high() {
    let env = Env::default();

    // Register mocks
    let token_id = env.register(MockToken, ());
    let pool_id = env.register(MockPool, ());

    let admin = Address::generate(&env);
    env.storage().instance().set(&ADMIN, &admin);
    env.storage().instance().set(&TOKEN, &token_id);
    env.storage().instance().set(&BLEND_POOL, &pool_id);

    // Supply only 500
    env.storage()
        .persistent()
        .set(&(env.current_contract_address(), 0u32), &500i128);

    let result = withdraw_amount_from_blend(env.clone(), 600);
    assert!(matches!(result, Err(Error::InsufficientFundsInBlend)));
}
