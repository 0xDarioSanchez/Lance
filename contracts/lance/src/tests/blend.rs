 #![cfg(test)]

// use super::*;
// use soroban_sdk::{
//     Address, Vec,contract, contractimpl, Env, Map
// };
// use soroban_sdk::testutils::Address as TestAddress; // for generate

// use crate::methods::blend::*;

// use blend_contract_sdk::pool;
// use crate::methods::balance::*;
// use crate::storage::{
//     error::Error, storage::DataKey,
// };


// const TOKEN: DataKey = DataKey::Token;
// const BLEND_POOL: DataKey = DataKey::BlendPool;
// const ADMIN: DataKey = DataKey::Admin; 
// const TOTAL_PRINCIPAL: DataKey = DataKey::TotalPrincipal;

// // Blend pool request type codes
// const BLEND_SUPPLY_REQUEST: u32 = 0;
// const BLEND_WITHDRAW_REQUEST: u32 = 1;
// const BLEND_BORROW_REQUEST: u32 = 2;
// const BLEND_REPAY_REQUEST: u32 = 3;
// const DEFAULT_RESERVE_ID: u32 = 0; 

// // ---------------- MOCK TOKEN ----------------
// #[contract]
// pub struct MockToken;

// #[contractimpl]
// impl MockToken {
//     pub fn balance(env: Env, addr: Address) -> i128 {
//         // give contract 1000 tokens balance for testing
//         if addr == env.current_contract_address() {
//             1000
//         } else {
//             0
//         }
//     }
// }

// // ---------------- MOCK BLEND POOL ----------------
// #[contract]
// pub struct MockPool;

// #[contractimpl]
// impl MockPool {
//     pub fn get_positions(env: Env, _addr: Address) -> pool::Positions {
//         let mut supply = Map::new(&env);
//         let mut collateral = Map::new(&env);
//         let mut liabilities = Map::new(&env);
//         supply.set(0u32, 500i128);

//         pool::Positions {
//             supply,
//             collateral,   // empty
//             liabilities,  // empty
//         }
//     }

//     pub fn submit_with_allowance(
//         _env: Env,
//         _from: Address,
//         _spender: Address,
//         _to: Address,
//         _reqs: Vec<pool::Request>,
//     ) {
//         // stub: do nothing
//     }
// }

// #[test]
// fn test_set_and_get_blend_pool() {
//     let env = Env::default();

//     let pool_addr = Address::generate(&env);
//     set_blend_pool(&env, &pool_addr);

//     let stored = get_blend_pool(&env).unwrap();
//     assert_eq!(stored, pool_addr);
// }

// #[test]
// fn test_lend_to_blend_no_tokens() {
//     let env = Env::default();

//     // setup admin + storage
//     let admin: Address = Address::generate(&env);

//     env.storage().instance().set(&ADMIN, &admin);
//     env.storage().instance().set(&TOKEN, &Address::generate(&env));
//     env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

//     // balance is 0 → should fail
//     let result = lend_to_blend(env.clone());
//     assert!(matches!(result, Err(Error::NoTokensToLend)));
// }

// #[test]
// fn test_withdraw_from_blend_no_position() {
//     let env = Env::default();

//     let admin = Address::generate(&env);
//     env.storage().instance().set(&ADMIN, &admin);
//     env.storage().instance().set(&TOKEN, &Address::generate(&env));
//     env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

//     // no supply position → should fail
//     let result = withdraw_from_blend(env.clone());
//     assert!(matches!(result, Err(Error::NoPositionInBlend)));
// }

// #[test]
// fn test_withdraw_amount_invalid() {
//     let env = Env::default();

//     let admin = Address::generate(&env);
//     env.storage().instance().set(&ADMIN, &admin);
//     env.storage().instance().set(&TOKEN, &Address::generate(&env));
//     env.storage().instance().set(&BLEND_POOL, &Address::generate(&env));

//     // amount <= 0 should fail
//     let result = withdraw_amount_from_blend(env.clone(), 0);
//     assert!(matches!(result, Err(Error::InvalidAmount)));
// }

// #[test]
// fn test_lend_to_blend_success() {
//     let env = Env::default();

//     // Register mocks
//     let token_id = env.register(MockToken, ());
//     let pool_id = env.register(MockPool, ());

//     // Setup storage
//     let admin = Address::generate(&env);
//     env.storage().instance().set(&ADMIN, &admin);
//     env.storage().instance().set(&TOKEN, &token_id);
//     env.storage().instance().set(&BLEND_POOL, &pool_id);

//     // Run function
//     let result = lend_to_blend(env.clone()).unwrap();
//     assert_eq!(result, 1000);
// }

// #[test]
// fn test_withdraw_from_blend_success() {
//     let env = Env::default();

//     // Register mocks
//     let token_id = env.register(MockToken, ());
//     let pool_id = env.register(MockPool, ());

//     // Setup storage
//     let admin = Address::generate(&env);
//     env.storage().instance().set(&ADMIN, &admin);
//     env.storage().instance().set(&TOKEN, &token_id);
//     env.storage().instance().set(&BLEND_POOL, &pool_id);

//     // Run function
//     let result = withdraw_from_blend(env.clone()).unwrap();
//     assert_eq!(result, 500);
// }

// #[test]
// fn test_withdraw_amount_too_high() {
//     let env = Env::default();

//     // Register mocks
//     let token_id = env.register(MockToken, ());
//     let pool_id = env.register(MockPool, ());

//     let admin = Address::generate(&env);
//     env.storage().instance().set(&ADMIN, &admin);
//     env.storage().instance().set(&TOKEN, &token_id);
//     env.storage().instance().set(&BLEND_POOL, &pool_id);

//     // Try to withdraw more than 500
//     let result = withdraw_amount_from_blend(env.clone(), 600);
//     assert!(matches!(result, Err(Error::InsufficientFundsInBlend)));
// }
