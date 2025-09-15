use soroban_sdk::{contracttype, Address, String, Vec, Env};
use super::{dispute_status::DisputaStat};
use crate::storage::{error::Error, storage::DataKey};


#[derive(Clone)]
#[contracttype]
pub struct Dispute {
    id: u32,
    dispute_id: u32,
    jury_members: Vec<Address>,
    // votes: Vec<Vote>,
    dispute_status: DisputaStat,
    initial_timestamp: u64,
    finish_timestamp: Option<u64>,
    employee: Address,
    employer: Address,  
    winner: Option<Address>,
    employee_proves: String,      
    employer_proves: Option<String>,
    payment: i128,
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