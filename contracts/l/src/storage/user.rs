use soroban_sdk::{contracttype, Address, String, Env};
use crate::storage::{error::Error, storage::DataKey};

#[derive(Clone)]
#[contracttype]
pub struct User {
    pub address: Address,
    pub is_employee: bool,
    pub is_employer: bool,
    pub is_judge: bool,
    pub personal_data: Option<String>,
}

pub(crate) fn get_user(env: &Env, user: Address) -> Result<User, Error> {
    let key = DataKey::Users(user);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::UserNotFound)
}

pub(crate) fn set_user(env: &Env, user: User) {
    let key = DataKey::Users(user.address.clone());

    env.storage().instance().set(&key, &user)
}
