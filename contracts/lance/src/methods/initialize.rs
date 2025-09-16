use soroban_sdk::{Address, Env};

use super::{
    admin::{has_admin, set_admin},
    // bend::set_blend_pool,
    token::set_token,
};
use crate::storage::error::Error;

pub fn initialize(
    env: &Env,
    admin: Address,
    token: Address,
    blend_pool: Address,
) -> Result<(), Error> {
    if has_admin(env) {
        return Err(Error::ContractInitialized);
    }

    set_admin(&env, &admin);
    set_token(&env, &token);
    // set_blend_pool(&env, &blend_pool);

    // events::contract::contract_initialized(&env, &admin); //TODO

    Ok(())
}
