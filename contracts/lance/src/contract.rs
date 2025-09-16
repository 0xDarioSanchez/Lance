use soroban_sdk::{contract, contractimpl, Env, String, Address};
use crate::methods::{
    dispute::*,
    service::*,
    balance::*,
    vote::*,
};
use crate::storage::vote::Vote;
use crate::storage::{
    error::Error,
    user::*,
    service::*,
    dispute::*, 
};
use crate::methods::{
    initialize::initialize,
    //service::*
};

pub trait ContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address, blend_pool: Address) -> Result<(), Error>; // Keep as is

    fn new_user(
        env: Env,
        user: Address, // This is fine, but let's see the impl
        is_employee: bool,
        is_employer: bool,
        is_judge: bool,
        personal_data: Option<String>,
    ) -> Result<(), Error>;

    fn get_user(env: Env, user: Address,) -> Result<User, Error> ;

    fn create_service(
        env: &Env,
        creator: Address,
        employer: Address,
        id: u32,
        duration: u64,
        metadata: Option<String>,
        milestone_payment: i128
    ) -> Result<(), Error> ;

    fn get_service(
        env: Env, 
        id: u32
    ) -> Result<Service, Error>;

    fn accept_service(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error>;

       fn approve_milestone(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error>;

   fn approve_service(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error>;

    fn create_dispute(
        env: &Env,
        creator: Address,
        id: u32,
        reason: String,
    ) -> Result<Dispute, Error> ;

    fn get_balance(env: &Env, 
        employee: Address
    ) -> i128;

    fn redeem(
        env: &Env,
        employee: Address,
    ) -> Result<i128, Error>;

    fn voter_registration(
        env: &Env, 
        creator: Address, 
        dispute_id: u32) -> Result<Dispute, Error>;

    fn vote(
        env: &Env, 
        creator: Address, 
        dispute_id: u32, 
        vote: Vote) -> Result<Dispute, Error>;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl ContractTrait for Contract {
    fn __constructor(env: Env, admin: Address, token: Address, blend_pool: Address) -> Result<(), Error> {
        initialize(&env, admin, token, blend_pool)
    }

    fn new_user(
        env: Env,
        user: Address, // The implementation takes Address by value
        is_employee: bool,
        is_employer: bool,
        is_judge: bool,
        personal_data: Option<String>,
    ) -> Result<(), Error> {
        set_user(&env, user, is_employee, is_employer, is_judge, personal_data);
        Ok(())
    }

    fn get_user(env: Env, user: Address,) -> Result<User, Error> {
        get_user(&env, user)
    }

    fn get_service(env: Env, id: u32) -> Result<Service, Error> {
        get_service(&env, id)
    }

    fn create_service(
        env: &Env,
        creator: Address,
        employer: Address,
        id: u32,
        duration: u64,
        metadata: Option<String>,
        milestone_payment: i128
    ) -> Result<(), Error> {
        create_service(env, creator, employer, id, duration, metadata, milestone_payment)?;
        Ok(())
    }  
            
   fn accept_service(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error> {
        accept_service(env, employer, id)   
   }

   fn approve_milestone(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error> {
        approve_milestone(env, employer, id)   
   }

   fn get_balance(env: &Env, 
        employee: Address
    ) -> i128 {
        get_balance(env, &employee)
   }

    // Approve the service completion from the employer, increasing employee balance and deleting the service.    
   fn approve_service(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error> {
        approve_service(env, employer, id)   
    }
    
    // Create a dispute for a service, can be created by either party involved in the service.  
    fn create_dispute(
          env: &Env,
          creator: Address,
          id: u32,
          proof: String,
     ) -> Result<Dispute, Error> {
          create_dispute(env, creator, id, proof)
    } 
    
    // Redeem the balance for the employee, setting it to zero and returning the amount to be transferred.
    fn redeem(
        env: &Env,
        employee: Address,
    ) -> Result<i128, Error> {
        redeem(env, employee)   
    }

    // Register as a voter for a dispute, can be called by any judge.
    fn voter_registration(
        env: &Env, 
        creator: Address, 
        dispute_id: u32) -> Result<Dispute, Error> {
        voter_registration(env, creator, dispute_id)
    }

    // Cast a vote for a dispute, can be called by any registered voter.
    fn vote(
        env: &Env, 
        creator: Address, 
        dispute_id: u32, 
        user_vote: Vote) -> Result<Dispute, Error> {
        vote(env, creator, dispute_id, user_vote)
    } 
}