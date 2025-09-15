use soroban_sdk::{token, Address, Env, String};

use crate::methods::balance::*;
use crate::storage::{
    service_status::ServiceStatus, error::Error, service::*, constants::*};

use crate::methods::token::token_transfer;

pub fn create_service(
    env: &Env,
    creator: Address,
    employer: Address,
    id: u32,
    duration: u64, // in days
    metadata: Option<String>,
    milestone_payment: i128
) -> Result<Service, Error> {
    creator.require_auth();

    if duration < 1 {
        return Err(Error::InvalidDuration);
    }

    let duration_in_seconds = duration * SECONDS_PER_DAY; // convert days to seconds

    let service = Service {
        id,
        metadata,
        employee: creator.clone(),
        employer,
        duration : duration_in_seconds,
        started_moment: 0,
        status: ServiceStatus::CREATED,
        current_milestone: 1,
        milestone_payment,  
    };

    set_service(env, id, service.clone());

    //TODO add event

    Ok(service)
}   

pub fn accept_service(
    env: &Env,
    employer: Address,
    id: u32
) -> Result<Service, Error> {
    employer.require_auth();

    let mut service = get_service(env, id)?;

    if service.employer != employer {
        return Err(Error::NotAuthorized);
    }

    service.started_moment = env.ledger().timestamp();

    // let token = crate::methods::token::get_token(env)?;

    token_transfer(
        env,
        &employer,
        &env.current_contract_address(),
        &service.milestone_payment
    )?;

    service.status = ServiceStatus::ACCEPTED;

    set_service(env, id, service.clone());

    //TODO add event

    Ok(service)
}

pub fn approve_milestone(
    env: &Env,
    employer: Address,
    id: u32,
) -> Result<Service, Error> {
    employer.require_auth();

    let mut service = get_service(env, id)?;

    if service.employer != employer {
        return Err(Error::NotAuthorized);
    }

    // if service.started_moment + service.duration < env.ledger().timestamp() {
    //     return Err(Error::InsufficientTime);
    // }

    token_transfer(
        env,
        &employer,
        &env.current_contract_address(),
        &service.milestone_payment
    )?;

    service.current_milestone += 1;
    service.status = ServiceStatus::WAITING;

    set_service(env, id, service.clone());

    //TODO add event

    Ok(service)
}


pub fn approve_service(
    env: &Env,
    employer: Address,
    id: u32,
) -> Result<Service, Error> {
    employer.require_auth();

    let mut service = get_service(env, id)?;

    if service.employer != employer {
        return Err(Error::NotAuthorized);
    }

    // if service.started_moment + service.duration < env.ledger().timestamp() {
    //     return Err(Error::InsufficientTime);
    // }

    token_transfer(
        env,
        &employer,
        &env.current_contract_address(),
        &service.milestone_payment
    )?;

    remove_service(env, id);

    //TODO add event

    Ok(service)
}

pub fn redeem(
    env: &Env,
    employee: Address,
) -> Result<i128, Error> {
    employee.require_auth();

    let balance = get_balance(env, &employee);

    if balance == 0 {
        return Err(Error::BalanceIsZero);
    }

    set_balance(env, &employee, 0);

    token_transfer(
        env,
        &env.current_contract_address(),
        &employee,
        &balance
    )?;

    Ok(balance)
}

//TODO add convertion
// pub fn convert_and_redeem(
//     env: &Env,
//     employee: Address,
//     conversion_rate: i128
// ) -> Result<i128, Error> {
//     employee.require_auth();

//     let balance = get_balance(env, &employee);

//     if balance == 0 {
//         return Err(Error::BalanceIsZero);
//     }

//     let converted_amount = balance * conversion_rate;

//     set_balance(env, &employee, 0);

//     token_transfer(
//         env,
//         &env.current_contract_address(),
//         &employee,
//         &converted_amount
//     )?;

//     Ok(converted_amount)
// }                   