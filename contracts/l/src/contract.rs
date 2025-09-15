use soroban_sdk::{contract, contractimpl, Env, String, Address};
use crate::methods::service::*;
use crate::storage::{
    error::Error,
    user::*,
    service::*,
    service_status::ServiceStatus
};
use crate::methods::{
    initialize::initialize,
    //service::*
};

pub trait ContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error>;

    fn new_user(
        env: Env,
        user: Address,
        is_employee: bool,
        is_employer: bool,
        is_judge: bool,
        personal_data: Option<String>,
    ) -> Result<(), Error> ;

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

    fn accept_service(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error>;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl ContractTrait for Contract {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin, token)
    }

    fn new_user(
        env: Env,
        user: Address,
        is_employee: bool,
        is_employer: bool,
        is_judge: bool,
        personal_data: Option<String>,
    ) -> Result<(), Error> {
        set_user(env, user, is_employee, is_employer, is_judge, personal_data);
        Ok(())
    }

    fn get_user(env: Env, user: Address,) -> Result<User, Error> {
        get_user(&env, user)
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
        creator.require_auth();

        create_service(env, creator, employer, id, duration, metadata, milestone_payment);
        Ok(())
    }  
            
   fn accept_service(
        env: &Env,
        employer: Address,
        id: u32
    ) -> Result<Service, Error> {
        accept_service(env, employer, id)   
   }

}