use soroban_sdk::{Address, Env, String, Vec};
use crate::storage::{error::Error, dispute::Dispute, service::*,
    dispute_status::DisputeStatus, storage::DataKey, service_status::ServiceStatus,
    dispute::set_dispute
};

pub fn create_dispute(
    env: &Env,
    creator: Address,
    service_id: u32,
    proof: String
) -> Result<Dispute, Error> {
    creator.require_auth();

    let service = get_service(env, service_id)?;
    let employee = service.employee;
    let employer = service.employer;

    if creator != employee && creator != employer {
        return Err(Error::NotAuthorized);
    }   
    if service.status != ServiceStatus::ACCEPTED {
        return Err(Error::InvalidServiceStatus);
    }   
    if service.status != ServiceStatus::DISPUTING {
        return Err(Error::DisputeAlreadyCreated);
    }   

    let current_id = env.storage().instance().get::<_, u32>(&DataKey::DisputeId).unwrap_or(0);
    let dispute_id = current_id + 1;
    env.storage().instance().set(&DataKey::DisputeId, &dispute_id);

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

    set_dispute(env, dispute_id, dispute.clone());

    //TODO add event

    Ok(dispute)
}