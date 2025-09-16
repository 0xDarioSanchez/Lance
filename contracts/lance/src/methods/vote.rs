use crate::{
    methods::balance::{get_balance, set_balance},
    storage::{
        dispute::{get_dispute, set_dispute, Dispute},
        dispute_status::DisputeStatus,
        error::Error,
        user::get_user,
        vote::Vote,
    },
};
use soroban_sdk::{Address, Env};

const VOTE_BASE_POWER: u32 = 1;
const MIN_VOTES_TO_FINISH_DISPUTE: u32 = 5;

pub fn voter_registration(env: &Env, creator: Address, dispute_id: u32) -> Result<Dispute, Error> {
    creator.require_auth();

    let new_judge_caller = get_user(env, creator.clone())?;

    if !new_judge_caller.is_judge {
        return Err(Error::NotAuthorized);
    }

    let mut dispute = get_dispute(env, dispute_id)?;

    if dispute.dispute_status != DisputeStatus::OPEN {
        return Err(Error::InvalidDisputeStatus);
    }

    if creator == dispute.employee || creator == dispute.employer {
        return Err(Error::NotAuthorized);
    }

    // TODO: Validate if user already exist
    dispute.jury_members.push_back(creator);
    set_dispute(env, dispute_id, dispute.clone());


    Ok(dispute)
}

pub fn vote(env: &Env, creator: Address, dispute_id: u32, vote: Vote) -> Result<Dispute, Error> {
    creator.require_auth();

    let mut dispute = get_dispute(env, dispute_id)?;

    if dispute.dispute_status != DisputeStatus::OPEN {
        return Err(Error::InvalidDisputeStatus);
    }

    if !dispute.jury_members.contains(&creator) {
        return Err(Error::NotAuthorized);
    }

    // TODO: Validate if vote already exist
    dispute.votes.push_back(vote);
    if dispute.votes.len() == MIN_VOTES_TO_FINISH_DISPUTE {
        dispute.dispute_status = DisputeStatus::EXECUTED;
        dispute.finish_timestamp = Some(env.ledger().timestamp());

        let mut trues_counter = 0;
        let mut falses_counter = 0;

        for vote in dispute.votes.iter() {
            if vote.vote {
                trues_counter += calculate_power(env, &vote.account)?;
            } else {
                falses_counter += calculate_power(env, &vote.account)?;
            }
        }

        if trues_counter < falses_counter {
            dispute.winner = Some(dispute.employee.clone());
            let balance = get_balance(env, &dispute.employee);
            set_balance(env, &dispute.employee, balance + dispute.payment);
        } else {
            dispute.winner = Some(dispute.employer.clone());
            let balance = get_balance(env, &dispute.employer);
            set_balance(env, &dispute.employer, balance + dispute.payment);
        }
    }

    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}

fn calculate_power(env: &Env, user: &Address) -> Result<u32, Error> {
    let u = get_user(env, user.clone())?;

    if u.delegates.len() == 0 {
        return Ok(VOTE_BASE_POWER);
    }

    let split = VOTE_BASE_POWER / (u.delegates.len() as u32);
    let mut total = 0u32;

    for d in u.delegates.iter() {
        total += calculate_power(env, &d)?;
    }

    Ok(total + split)
}
