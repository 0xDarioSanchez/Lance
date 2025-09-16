use crate::{ storage::{
    dispute::{get_dispute, set_dispute, Dispute},
    dispute_status::DisputeStatus,
    error::Error,
    service::*,
    service_status::ServiceStatus,
    storage::DataKey,
}};
use crate::events::event::created_dispute;
use soroban_sdk::{Address, Env, String, Vec};

pub fn create_dispute(
    env: &Env,
    creator: Address,
    service_id: u32,
    proof: String,
) -> Result<Dispute, Error> {
    creator.require_auth();

    let mut service = get_service(env, service_id)?;
    let employee = service.employee.clone();
    let employer = service.employer.clone();

    if creator != employer {
        return Err(Error::NotAuthorized);
    }
    if service.status != ServiceStatus::ACCEPTED {
        return Err(Error::InvalidServiceStatus);
    }
    if service.status != ServiceStatus::DISPUTING {
        return Err(Error::DisputeAlreadyCreated);
    }

    let current_id = env
        .storage()
        .instance()
        .get::<_, u32>(&DataKey::DisputeId)
        .unwrap_or(0);
    let dispute_id = current_id + 1;
    env.storage()
        .instance()
        .set(&DataKey::DisputeId, &dispute_id);

    let dispute = Dispute {
        dispute_id,
        jury_members: Vec::new(env),
        votes: Vec::new(env),
        dispute_status: DisputeStatus::OPEN,
        initial_timestamp: env.ledger().timestamp(),
        finish_timestamp: None,
        employee: employee.clone(),
        employer: employer.clone(),
        winner: None,
        employee_proves: None,
        employer_proves: proof,
        payment: service.milestone_payment,
    };

    service.status = ServiceStatus::DISPUTING;
    set_service(env, service_id, service.clone());
    set_dispute(env, dispute_id, dispute.clone());

    created_dispute(&env, &creator, &service_id);
    
    Ok(dispute)
}

pub fn update_dispute(env: &Env, dispute_id: u32, proof: String) -> Result<Dispute, Error> {
    let mut dispute = get_dispute(env, dispute_id)?;
    let employee = dispute.employee.clone();

    employee.require_auth();

    if dispute.dispute_status != DisputeStatus::OPEN {
        return Err(Error::InvalidDisputeStatus);
    }

    dispute.employee_proves = core::prelude::v1::Some(proof);

    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}

