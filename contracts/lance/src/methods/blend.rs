use soroban_sdk::{token, Address, Env, Symbol, Vec, vec};
use blend_contract_sdk::pool;
use crate::methods::balance::*;
use crate::storage::{
    constants::*, error::Error, service::*, service_status::ServiceStatus, storage::DataKey,
};

use crate::methods::token::token_transfer;

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

pub(crate) fn set_blend_pool(env: &Env, blend_pool: &Address) {
    let key = DataKey::BlendPool;
    env.storage().instance().set(&key, blend_pool);
}

pub(crate) fn get_blend_pool(env: &Env) -> Result<Address, Error> {
    let key = DataKey::BlendPool;
    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::ContractNotInitialized)
}

/// Supply (lend) all tokens held by the contract into the Blend pool.
/// Only callable by admin.
pub fn lend_to_blend(env: Env) -> Result<i128, Error> {

    let admin: Address = env
        .storage()
        .instance()
        .get(&ADMIN)
        .ok_or(Error::ContractNotInitialized)?;
    admin.require_auth();

    let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
    let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

    let token = token::Client::new(&env, &token_address);
    let blend_pool = pool::Client::new(&env, &blend_pool_address);

    // Get current contract token balance
    let contract_balance = token.balance(&env.current_contract_address());

    if contract_balance <= 0 {
        // TODO: implement a minimal contract idle buffer if you want instant payouts
        return Err(Error::NoTokensToLend);
    }

    // Build supply request (SupplyCollateral)
    let supply_request = pool::Request {
        request_type: BLEND_SUPPLY_REQUEST, // keep your constant
        address: token_address.clone(),
        amount: contract_balance,
    };

    let requests = Vec::from_array(&env, [supply_request]);

    // Use submit_with_allowance: pool will pull tokens (allowance) and mint bTokens to this contract.
    // This avoids doing an explicit token.transfer and avoids needing `authorize_as_current_contract`.
    blend_pool.submit_with_allowance(
        &env.current_contract_address(), // from (this contract is the source of funds / allowance)
        &env.current_contract_address(), // spender (this contract)
        &env.current_contract_address(), // recipient of bTokens (this contract)
        &requests,
    );

    // Emit lending event
    env.events().publish(
        (
            Symbol::new(&env, "lent_to_blend"),
            env.current_contract_address(),
        ),
        contract_balance,
    );

    Ok(contract_balance)
}

/// Withdraw the entire position from Blend back to this contract.
/// Only callable by admin.
pub fn withdraw_from_blend(env: Env) -> Result<i128, Error> {
    // --- admin-only ---
    let admin: Address = env
        .storage()
        .instance()
        .get(&ADMIN)
        .ok_or(Error::ContractNotInitialized)?;
    admin.require_auth();

    let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
    let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

    let blend_pool = pool::Client::new(&env, &blend_pool_address);

    // Get current positions (bToken supply/pool position) for this contract
    let positions = blend_pool.get_positions(&env.current_contract_address());

    // Assume reserve_id 0 or DEFAULT_RESERVE_ID; adjust as your pool config requires
    let total_supply = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

    if total_supply <= 0 {
        return Err(Error::NoPositionInBlend);
    }

    // Create withdrawal request for entire position
    let withdraw_request = pool::Request {
        request_type: BLEND_WITHDRAW_REQUEST,
        address: token_address.clone(),
        amount: total_supply,
    };

    let requests = Vec::from_array(&env, [withdraw_request]);

    // Use submit_with_allowance so pool can burn bTokens and transfer tokens back to this contract
    blend_pool.submit_with_allowance(
        &env.current_contract_address(), // from (this contract holds the bTokens)
        &env.current_contract_address(), // spender
        &env.current_contract_address(), // recipient (this contract)
        &requests,
    );

    // Emit withdrawal event
    env.events().publish(
        (
            Symbol::new(&env, "withdrawn_from_blend"),
            env.current_contract_address(),
        ),
        total_supply,
    );

    Ok(total_supply)
}

/// Withdraw a specific amount from Blend back to this contract.
/// Only callable by admin.
pub fn withdraw_amount_from_blend(env: Env, amount: i128) -> Result<i128, Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    // --- admin-only ---
    let admin: Address = env
        .storage()
        .instance()
        .get(&ADMIN)
        .ok_or(Error::ContractNotInitialized)?;
    admin.require_auth();

    let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
    let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

    let blend_pool = pool::Client::new(&env, &blend_pool_address);

    // Get current positions to check available bToken supply
    let positions = blend_pool.get_positions(&env.current_contract_address());
    let total_supply = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

    if total_supply <= 0 {
        return Err(Error::NoPositionInBlend);
    }

    if amount > total_supply {
        return Err(Error::InsufficientFundsInBlend);
    }

    // Create withdrawal request for specified amount
    let withdraw_request = pool::Request {
        request_type: BLEND_WITHDRAW_REQUEST, // use the constant instead of magic number
        address: token_address.clone(),
        amount,
    };

    let requests = Vec::from_array(&env, [withdraw_request]);

    // Pool will burn bTokens and send tokens back to this contract
    blend_pool.submit_with_allowance(
        &env.current_contract_address(), // from (this contract)
        &env.current_contract_address(), // spender
        &env.current_contract_address(), // recipient
        &requests,
    );

    // Emit withdrawal event
    env.events().publish(
        (
            Symbol::new(&env, "withdrawn_amount_from_blend"),
            env.current_contract_address(),
        ),
        amount,
    );

    Ok(amount)
}
