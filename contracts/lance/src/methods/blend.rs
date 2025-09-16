use soroban_sdk::{token, Address, Env};

use crate::methods::balance::*;
use crate::storage::{
    constants::*, error::Error, service::*, service_status::ServiceStatus, storage::DataKey,
};

use crate::methods::token::token_transfer;

const TOKEN: DataKey = DataKey::Token;
const BLEND_POOL: DataKey = DataKey::BlendPool;

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

pub fn lend_to_blend(env: Env) -> i128 {
    let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
    let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

    let token = token::Client::new(&env, &token_address);
    let blend_pool = pool::Client::new(&env, &blend_pool_address);

    // Get current contract balance
    let contract_balance = token.balance(&env.current_contract_address());

    if contract_balance <= 0 {
        //TODO implement a minimal contract balance to assure liquidity for instant user payments
        panic_with_error!(&env, &EscrowError::NoTokensToLend);
    }

    env.authorize_as_current_contract(vec![
        &env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_address.clone(),
                fn_name: Symbol::new(&env, "transfer"),
                args: (
                    env.current_contract_address(),
                    blend_pool_address.clone(),
                    contract_balance,
                )
                    .into_val(&env),
            },
            sub_invocations: vec![&env],
        }),
    ]);

    let supply_request = pool::Request {
        request_type: BLEND_SUPPLY_REQUEST,
        address: token_address.clone(),
        amount: contract_balance,
    };

    let requests = Vec::from_array(&env, [supply_request]);

    blend_pool.submit(
        &env.current_contract_address(), // from (this contract)
        &env.current_contract_address(), // spender (this contract)
        &env.current_contract_address(), // to (bTokens recipient - this contract)
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

    contract_balance
}

pub fn withdraw_from_blend(env: Env) -> i128 {
    let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
    let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

    let blend_pool = pool::Client::new(&env, &blend_pool_address);

    // Get current positions to withdraw entire balance
    let positions = blend_pool.get_positions(&env.current_contract_address());
    let total_supply = positions.supply.get(0).unwrap_or(0); // Assuming reserve_id 0, adjust as needed

    if total_supply <= 0 {
        panic_with_error!(&env, &EscrowError::NoPositionInBlend);
    }

    // Create withdrawal request for entire position
    let withdraw_request = pool::Request {
        request_type: BLEND_WITHDRAW_REQUEST,
        address: token_address.clone(),
        amount: total_supply,
    };

    let requests = Vec::from_array(&env, [withdraw_request]);

    // Submit withdrawal request
    blend_pool.submit(
        &env.current_contract_address(), // from (this contract)
        &env.current_contract_address(), // spender (this contract)
        &env.current_contract_address(), // to (withdrawal recipient - this contract)
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

    total_supply
}

pub fn withdraw_amount_from_blend(env: Env, amount: i128) -> i128 {
    if amount <= 0 {
        panic_with_error!(&env, &EscrowError::AmountMustBePositive);
    }

    let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
    let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

    let blend_pool = pool::Client::new(&env, &blend_pool_address);

    // Get current positions to check available balance
    let positions = blend_pool.get_positions(&env.current_contract_address());
    let total_supply = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

    if total_supply <= 0 {
        panic_with_error!(&env, &EscrowError::NoPositionInBlend);
    }

    if amount > total_supply {
        panic_with_error!(&env, &EscrowError::InsufficientFundsInBlend);
    }

    // Create withdrawal request for specified amount
    let withdraw_request = pool::Request {
        request_type: 1, // Withdraw request type
        address: token_address.clone(),
        amount: amount,
    };

    let requests = Vec::from_array(&env, [withdraw_request]);

    // Submit withdrawal request
    blend_pool.submit(
        &env.current_contract_address(), // from (this contract)
        &env.current_contract_address(), // spender (this contract)
        &env.current_contract_address(), // to (withdrawal recipient - this contract)
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

    amount
}
