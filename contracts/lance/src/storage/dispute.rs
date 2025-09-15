use soroban_sdk::{contracttype, Address, String, Vec, Env};
use super::{dispute_status::DisputeStatus, vote::Vote};
use crate::storage::{error::Error, storage::DataKey};


#[derive(Clone)]
#[contracttype]
pub struct Dispute {
    pub dispute_id: u32,
    pub jury_members: Vec<Address>,
    pub votes: Vec<Vote>,
    pub dispute_status: DisputeStatus,
    pub initial_timestamp: u64,
    pub finish_timestamp: Option<u64>,
    pub employee: Address,
    pub employer: Address,  
    pub winner: Option<Address>,
    pub employee_proves: Option<String>,      
    pub employer_proves: String,
    pub payment: i128,
}

pub(crate) fn get_dispute(env: &Env, dispute_id: u32) -> Result<Dispute, Error> {
    let key = DataKey::Disputes(dispute_id);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::DisputeNotFound)
}

pub(crate) fn set_dispute(env: &Env, dispute_id: u32, dispute: Dispute) {
    let key = DataKey::Disputes(dispute_id);

    env.storage().instance().set(&key, &dispute)
}