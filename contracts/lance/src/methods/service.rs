use soroban_sdk::{Env, Address, String};

use crate::storage::{
    service_status::ServiceStatus, error::Error, service::*, constants::*,
};

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
        buy_moment: env.ledger().timestamp(),
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

    //TODO token transfer from employer to contract

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

    if service.buy_moment + service.duration < env.ledger().timestamp() {
        return Err(Error::InsufficientTime);
    }

    //TODO token transfer from contract to employee

    service.current_milestone += 1;
    service.status = ServiceStatus::WAITING;

    set_service(env, id, service.clone());

    //TODO add event

    Ok(service)
}
